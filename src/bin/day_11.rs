use async_std::task;

use adventofcode_2019::intcode_computer::*;

use std::collections::HashMap;
use std::convert::{Into, TryFrom};
use std::default::Default;
use std::ops::{Deref, DerefMut};
use std::{fmt, io, mem};

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();

    let grid = task::block_on(run_painting(&input, PanelColor::Black.into()));
    println!("Part 1: {}", grid.len());

    let grid = task::block_on(run_painting(&input, PanelColor::White.into()));
    println!("Part 2: \n{}", grid);
}

async fn run_painting(program: &str, initial_input: Int) -> Grid {
    let ((input_sender, input_receiver), (output_sender, output_receiver)) =
        IntcodeComputer::create_io();
    let mut computer = IntcodeComputer::new(&input_receiver, &output_sender);

    computer.init(program).unwrap();

    let t = task::spawn(async move {
        let mut robot = HullPaintingRobot::default();
        let mut grid = Grid::new();

        input_sender.send(initial_input).await;
        while let Some(new_color) = output_receiver.recv().await {
            let turn_input = output_receiver.recv().await.unwrap();

            grid.insert(robot.pos.clone(), PanelColor::try_from(new_color).unwrap());
            robot.turn(turn_input).unwrap();
            robot.move_forward();

            let panel_color = match grid.get(&robot.pos) {
                Some(color) => *color,
                _ => PanelColor::default(),
            };

            input_sender.send((panel_color).into()).await;
        }

        grid
    });

    task::block_on(async {
        computer.run().await.unwrap();
    });
    mem::drop(computer);
    mem::drop(output_sender);

    t.await
}

struct Grid(HashMap<Coord, PanelColor>);

impl Grid {
    fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Deref for Grid {
    type Target = HashMap<Coord, PanelColor>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for PanelColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PanelColor::*;

        write!(
            f,
            "{}",
            match self {
                White => "█",
                Black => " ",
            }
        )
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = self.keys().fold(std::i64::MAX, |acc, c| acc.min(c.0));
        let max_x = self.keys().fold(std::i64::MIN, |acc, c| acc.max(c.0));
        let min_y = self.keys().fold(std::i64::MAX, |acc, c| acc.min(c.1));
        let max_y = self.keys().fold(std::i64::MIN, |acc, c| acc.max(c.1));

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let color = match self.get(&Coord(x, y)) {
                    Some(color) => *color,
                    _ => PanelColor::default(),
                };
                write!(f, "{}", color)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Coord(Int, Int);

#[derive(Debug, Clone, Default)]
struct HullPaintingRobot {
    pos: Coord,
    facing: Direction,
}

impl HullPaintingRobot {
    fn turn(&mut self, turn_input: Int) -> Result<(), Error> {
        self.facing = self.facing.turn(turn_input)?;
        Ok(())
    }

    fn move_forward(&mut self) {
        use Direction::*;

        let step = 1;

        match &self.facing {
            Up => self.pos.1 -= step,
            Right => self.pos.0 += step,
            Down => self.pos.1 += step,
            Left => self.pos.0 -= step,
        };
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

#[derive(Debug, Clone, Copy)]
enum PanelColor {
    Black,
    White,
}

impl Default for PanelColor {
    fn default() -> Self {
        PanelColor::Black
    }
}

impl TryFrom<Int> for PanelColor {
    type Error = Error;
    fn try_from(input: Int) -> Result<Self, Self::Error> {
        use PanelColor::*;

        match input {
            0 => Ok(Black),
            1 => Ok(White),
            x => Err(Error::InvalidInput(x)),
        }
    }
}

impl Into<Int> for PanelColor {
    fn into(self) -> Int {
        use PanelColor::*;

        match self {
            Black => 0,
            White => 1,
        }
    }
}

impl Direction {
    fn turn(&self, input: Int) -> Result<Self, Error> {
        use Direction::*;

        match input {
            0 => Ok(match self {
                // Turn left
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            }),
            1 => Ok(match self {
                // Turn right
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            }),
            x => Err(Error::InvalidInput(x)),
        }
    }
}

#[derive(Debug)]
enum Error {
    InvalidInput(Int),
}
