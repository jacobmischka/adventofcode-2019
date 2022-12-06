use async_std::task;

use adventofcode_2019::grid::*;
use adventofcode_2019::intcode_computer::*;

use std::{io, mem};

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();

    let grid = task::block_on(run_program(&input));
    println!("{}", grid);

    let intersections: Vec<&Coord> = grid
        .iter()
        .filter_map(|(coord, c)| {
            let x = coord.0;
            let y = coord.1;
            if *c == '#'
                && ((is_scaffold(&grid, x, y - 1) && is_scaffold(&grid, x, y + 1))
                    && (is_scaffold(&grid, x - 1, y) && is_scaffold(&grid, x + 1, y)))
            {
                Some(coord)
            } else {
                None
            }
        })
        .collect();

    let p1: i64 = intersections.into_iter().map(alignment_parameter).sum();
    println!("Part 1: {}", p1);
}

fn alignment_parameter(Coord(x, y): &Coord) -> i64 {
    x * y
}

fn is_scaffold(grid: &TileGrid, x: i64, y: i64) -> bool {
    *grid.get(&Coord(x, y)).unwrap_or(&'.') == '#'
}

type TileGrid = Grid<char>;

async fn run_program(program: &str) -> TileGrid {
    let ((_input_sender, input_receiver), (output_sender, output_receiver)) =
        IntcodeComputer::create_io();
    let mut computer = IntcodeComputer::new(&input_receiver, &output_sender);

    computer.init(program).unwrap();

    let t = task::spawn(async move {
        let mut grid = TileGrid::new();
        let mut x: Int = 0;
        let mut y: Int = 0;
        while let Ok(output) = output_receiver.recv().await {
            let c: char = (output as u8).into();
            match c {
                '\n' => {
                    x = 0;
                    y += 1;
                }
                c => {
                    grid.insert(Coord(x, y), c);
                    x += 1;
                }
            }
        }

        grid
    });

    computer.run().await.unwrap();
    mem::drop(computer);
    mem::drop(output_sender);

    t.await
}
