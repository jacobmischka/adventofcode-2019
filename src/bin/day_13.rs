use async_std::{
    sync::{Arc, RwLock},
    task,
};
use rustbox::{Color, Event, Key, RustBox, RB_NORMAL};

use adventofcode_2019::grid::*;
use adventofcode_2019::intcode_computer::*;

use std::convert::TryFrom;
use std::time::Duration;
use std::{fmt, io, mem};

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let mut input = line.trim().to_string();

    let grid = play(&input);

    let num_blocks = grid.values().fold(
        0,
        |acc, tile| if *tile == Tile::Block { acc + 1 } else { acc },
    );
    println!("Part 1: {}", num_blocks);

    input.replace_range(..1, "2");
    play(&input);
}

fn play(program: &str) -> GameGrid {
    let ((in_sender, in_receiver), (out_sender, out_receiver)) = IntcodeComputer::create_io();
    let mut computer = IntcodeComputer::new(&in_receiver, &out_sender);
    computer.init(program).unwrap();

    let rustbox = Arc::new(RustBox::init(Default::default()).expect("failed creating rustbox"));
    let run = Arc::new(RwLock::new(false));

    let rb = rustbox.clone();
    let running = run.clone();

    let io = task::spawn(async move {
        let wait_duration = Duration::new(1, 0);
        while *running.read().await {
            if cfg!(feature = "slowgamemode") {
                if let Ok(Event::KeyEvent(key)) = rb.poll_event(false) {
                    if let Some(input) = get_input(key) {
                        in_sender.send(input).await;
                    }
                }
            } else {
                if let Ok(Event::KeyEvent(key)) = rb.peek_event(wait_duration, false) {
                    if let Some(input) = get_input(key) {
                        in_sender.send(input).await;
                    }
                } else {
                    in_sender.send(0).await;
                }
            }
        }
    });

    let rb = rustbox.clone();
    let t = task::spawn(async move {
        let mut grid = GameGrid::new();
        let mut score = 0;

        while let Some(x) = out_receiver.recv().await {
            let y = out_receiver.recv().await.expect("failed getting y");
            let value = out_receiver
                .recv()
                .await
                .expect("failed getting output value");
            if x == -1 && y == 0 {
                score = value;
            } else {
                let tile = Tile::try_from(value).expect("failed creating tile");
                grid.insert(Coord(x, y), tile);
            }

            grid.draw(&rb);
            rb.print(
                0,
                0,
                RB_NORMAL,
                Color::White,
                Color::Black,
                &score.to_string(),
            );
            rb.present();
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
    task::block_on(io);

    task::block_on(t)
}

fn get_input(key: Key) -> Option<Int> {
    match key {
        Key::Right => Some(1),
        Key::Left => Some(-1),
        Key::Char(' ') => {
            if cfg!(feature = "slowgamemode") {
                Some(0)
            } else {
                None
            }
        }
        _ => None,
    }
}

type GameGrid = Grid<Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

impl TryFrom<Int> for Tile {
    type Error = Error;
    fn try_from(input: Int) -> Result<Self, Self::Error> {
        use Tile::*;

        match input {
            0 => Ok(Empty),
            1 => Ok(Wall),
            2 => Ok(Block),
            3 => Ok(HorizontalPaddle),
            4 => Ok(Ball),
            x => Err(Error::InvalidInput(x)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Tile::*;

        write!(
            f,
            "{}",
            match self {
                Empty => " ",
                Wall => "█",
                Block => "□",
                HorizontalPaddle => "―",
                Ball => "•",
            }
        )
    }
}

#[derive(Debug)]
enum Error {
    InvalidInput(Int),
}
