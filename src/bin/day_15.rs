use async_std::{
    sync::{Arc, RwLock, Sender},
    task,
};
use rustbox::{Event, Key, RustBox};

use adventofcode_2019::grid::*;
use adventofcode_2019::intcode_computer::*;

use std::convert::TryFrom;
use std::{fmt, io, mem, str::FromStr};

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();

    run(&input);
}

fn run(program: &str) -> TileGrid {
    let ((in_sender, in_receiver), (out_sender, out_receiver)) = IntcodeComputer::create_io();
    let mut computer = IntcodeComputer::new(&in_receiver, &out_sender);
    computer.init(program).unwrap();

    let rb = RustBox::init(Default::default()).expect("failed creating rustbox");
    let t = task::spawn(async move {
        let mut grid = TileGrid::new();
        let mut current_coord = Coord(40, 20);
        grid.insert(current_coord.clone(), Tile::Droid);
        grid.draw(&rb);
        rb.present();

        let mut movement_command = wait_for_movement_command(&rb).await;
        in_sender.send((&movement_command).into()).await;

        while let Some(x) = out_receiver.recv().await {
            let status = StatusCode::try_from(x).unwrap();
            dbg!(&status, &grid);
            match status {
                StatusCode::HitWall => {
                    grid.insert(movement_command.move_from(&current_coord), Tile::Wall);
                }
                StatusCode::MoveSuccess => {
                    *grid.get_mut(&current_coord).unwrap() = Tile::Empty;
                    current_coord = movement_command.move_from(&current_coord);
                    grid.insert(current_coord.clone(), Tile::Droid);
                }
                StatusCode::ReachedOxygenSystem => {
                    current_coord = movement_command.move_from(&current_coord);
                    grid.insert(current_coord.clone(), Tile::OxygenSystem);
                }
            };

            grid.draw(&rb);
            rb.present();
            movement_command = wait_for_movement_command(&rb).await;
            in_sender.send((&movement_command).into()).await;
        }

        grid
    });

    task::block_on(async {
        computer.run().await.expect("failed running computer");
    });
    mem::drop(computer);
    mem::drop(out_sender);

    task::block_on(t)
}

async fn wait_for_movement_command(rb: &RustBox) -> MovementCommand {
    loop {
        if let Ok(Some(movement_command)) = if let Ok(Event::KeyEvent(key)) = rb.poll_event(false) {
            Some(MovementCommand::try_from(key)).transpose()
        } else {
            Ok(None)
        } {
            return movement_command;
        }
    }
}

#[derive(Debug)]
enum MovementCommand {
    North,
    South,
    West,
    East,
}

impl MovementCommand {
    fn move_from(&self, c: &Coord) -> Coord {
        use MovementCommand::*;

        let mut c = c.clone();

        match self {
            North => {
                c.1 -= 1;
            }
            South => {
                c.1 += 1;
            }
            West => {
                c.0 -= 1;
            }
            East => {
                c.0 += 1;
            }
        }

        c
    }
}

impl Into<Int> for &MovementCommand {
    fn into(self) -> Int {
        use MovementCommand::*;

        match self {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }
}

impl TryFrom<Key> for MovementCommand {
    type Error = Error;
    fn try_from(key: Key) -> Result<Self, Self::Error> {
        use MovementCommand::*;
        match key {
            Key::Up => Ok(North),
            Key::Down => Ok(South),
            Key::Left => Ok(West),
            Key::Right => Ok(East),
            Key::Ctrl('c') => std::process::exit(0),
            k => Err(Error::InvalidKey(k)),
        }
    }
}

type TileGrid = Grid<Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unknown,
    Empty,
    Wall,
    Droid,
    OxygenSystem,
}

#[derive(Debug)]
enum StatusCode {
    HitWall,
    MoveSuccess,
    ReachedOxygenSystem,
}

impl TryFrom<Int> for StatusCode {
    type Error = Error;

    fn try_from(i: Int) -> Result<Self, Self::Error> {
        use StatusCode::*;

        match i {
            0 => Ok(HitWall),
            1 => Ok(MoveSuccess),
            2 => Ok(ReachedOxygenSystem),
            x => Err(Error::InvalidInput(x)),
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Unknown
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Tile::*;

        write!(
            f,
            "{}",
            match self {
                Unknown => "?",
                Empty => " ",
                Wall => "#",
                Droid => "D",
                OxygenSystem => "O",
            }
        )
    }
}

#[derive(Debug)]
enum Error {
    InvalidKey(Key),
    InvalidInput(Int),
}
