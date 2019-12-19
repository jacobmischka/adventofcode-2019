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

    let grid = run(&input);
}

fn run(program: &str) -> TileGrid {
    let ((in_sender, in_receiver), (out_sender, out_receiver)) = IntcodeComputer::create_io();
    let mut computer = IntcodeComputer::new(&in_receiver, &out_sender);
    computer.init(program).unwrap();

    let rustbox = Arc::new(RustBox::init(Default::default()).expect("failed creating rustbox"));
    let run = Arc::new(RwLock::new(false));

    let rb = rustbox.clone();
    let running = run.clone();

    let rb = rustbox.clone();
    let t = task::spawn(async move {
        let mut grid = TileGrid::new();
        let mut current_coord = Coord(40, 40);
        grid.insert(current_coord.clone(), Tile::Droid);

        let mut movement_command = wait_for_movement_command(&rb).await;
        in_sender.send(movement_command.into()).await;

        while let Some(x) = out_receiver.recv().await {
            let status = out_receiver.recv().await.expect("failed getting status");
            let value = out_receiver
                .recv()
                .await
                .expect("failed getting output value");

            grid.draw(&rb);
            rb.present();
            movement_command = wait_for_movement_command(&rb).await;
            in_sender.send(movement_command.into()).await;
        }

        grid
    });

    let running = run.clone();
    task::block_on(async {
        *running.write().await = true;
        computer.run().await.expect("failed running computer");
        *running.write().await = false;
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
    fn move_from(&self, c: &mut Coord) {
        use MovementCommand::*;

        match self {
            North => {
                c.0 -= 1;
            }
            South => {
                c.0 += 1;
            }
            West => {
                c.1 -= 1;
            }
            East => {
                c.1 += 1;
            }
        }
    }
}

impl Into<Int> for MovementCommand {
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
}

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
                Wall => "█",
                Droid => "🤖",
            }
        )
    }
}

#[derive(Debug)]
enum Error {
    InvalidKey(Key),
    InvalidInput(Int),
}
