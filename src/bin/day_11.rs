use async_std::task;

use adventofcode_2019::grid::*;
use adventofcode_2019::intcode_computer::*;

use std::convert::{Into, TryFrom};
use std::default::Default;
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

type PanelGrid = Grid<PanelColor>;

async fn run_painting(program: &str, initial_input: Int) -> PanelGrid {
    let ((input_sender, input_receiver), (output_sender, output_receiver)) =
        IntcodeComputer::create_io();
    let mut computer = IntcodeComputer::new(&input_receiver, &output_sender);

    computer.init(program).unwrap();

    let t = task::spawn(async move {
        let mut robot = HullPaintingRobot::default();
        let mut grid = PanelGrid::new();

        input_sender.send(initial_input).await.unwrap();
        while let Ok(new_color) = output_receiver.recv().await {
            let turn_input = output_receiver.recv().await.unwrap();

            grid.insert(robot.pos.clone(), PanelColor::try_from(new_color).unwrap());
            robot.turn(turn_input).unwrap();
            robot.move_forward();

            let panel_color = match grid.get(&robot.pos) {
                Some(color) => *color,
                _ => PanelColor::default(),
            };

            input_sender.send((panel_color).into()).await.unwrap();
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

#[derive(Debug, Clone, Default)]
struct HullPaintingRobot {
    pos: Coord,
    facing: Direction,
}

impl HullPaintingRobot {
    pub fn turn(&mut self, turn_input: Int) -> Result<(), Error> {
        self.facing = self.facing.turn(turn_input)?;
        Ok(())
    }

    pub fn move_forward(&mut self) {
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
