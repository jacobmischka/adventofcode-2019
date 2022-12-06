use async_std::{
    sync::{Arc, RwLock},
    task,
};
use clap::{App, Arg};
use rustbox::{Color, Event, Key, RustBox, RB_NORMAL};

use adventofcode_2019::grid::*;
use adventofcode_2019::intcode_computer::*;

use std::collections::VecDeque;
use std::convert::TryFrom;
use std::time::Duration;
use std::{fmt, fs, io, mem, str::FromStr};

fn main() {
    let matches = App::new("Intcode arcade brick breaker")
        .version("1.0")
        .arg(
            Arg::with_name("inputs-in")
                .help("where to load your previous inputs from")
                .short("i")
                .long("inputs-in")
                .value_name("FILE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("inputs-out")
                .help("where to dump your inputs")
                .short("o")
                .long("inputs-out")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let mut input = line.trim().to_string();

    let (grid, _inputs) = play(&input, vec![0].into());

    let num_blocks = grid.values().fold(
        0,
        |acc, tile| if *tile == Tile::Block { acc + 1 } else { acc },
    );
    println!("Part 1: {}", num_blocks);

    let loaded_inputs: VecDeque<Int> = match matches.value_of("inputs-in") {
        Some(path) => IntVec::from_str(
            fs::read_to_string(path)
                .expect("failed loading inputs file")
                .trim(),
        )
        .expect("failed deserializing inputs file")
        .into_inner()
        .into(),
        None => VecDeque::new(),
    };

    input.replace_range(..1, "2");
    let (_grid, inputs) = play(&input, loaded_inputs);

    if let Some(outpath) = matches.value_of("inputs-out") {
        fs::write(outpath, IntVec(inputs).to_string()).unwrap();
    }
}

fn play(program: &str, mut loaded_inputs: VecDeque<Int>) -> (GameGrid, Vec<Int>) {
    let ((in_sender, in_receiver), (out_sender, out_receiver)) = IntcodeComputer::create_io();
    let mut computer = IntcodeComputer::new(&in_receiver, &out_sender);
    computer.init(program).unwrap();

    let rustbox = Arc::new(RustBox::init(Default::default()).expect("failed creating rustbox"));
    let run = Arc::new(RwLock::new(false));

    let rb = rustbox.clone();
    let running = run.clone();

    let io = task::spawn(async move {
        let mut inputs: Vec<Int> = Vec::new();
        let wait_duration = Duration::from_millis(500);

        while *running.read().await {
            if let Some(input) = match loaded_inputs.pop_front() {
                Some(input) => Some(input),
                None => {
                    if cfg!(feature = "slowgamemode") {
                        if let Ok(Event::KeyEvent(key)) = rb.poll_event(false) {
                            get_input(key)
                        } else {
                            None
                        }
                    } else {
                        if let Ok(Event::KeyEvent(key)) = rb.peek_event(wait_duration, false) {
                            get_input(key)
                        } else {
                            Some(0)
                        }
                    }
                }
            } {
                inputs.push(input);
                in_sender.send(input).await.unwrap();
            };
        }

        inputs
    });

    let rb = rustbox.clone();
    let t = task::spawn(async move {
        let mut grid = GameGrid::new();
        let mut score = 0;

        while let Ok(x) = out_receiver.recv().await {
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

    (task::block_on(t), task::block_on(io))
}

fn get_input(key: Key) -> Option<Int> {
    match key {
        Key::Right => Some(1),
        Key::Left => Some(-1),
        Key::Ctrl('c') => std::process::exit(0),
        _ => Some(0),
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
