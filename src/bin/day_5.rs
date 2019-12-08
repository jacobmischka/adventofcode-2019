use async_std::task;

use adventofcode_2019::intcode_computer::*;

use std::io;

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();

    let mut computer = IntcodeComputer::new();

    computer.init(&input).unwrap();
    computer.add_input(1);
    task::block_on(computer.run()).unwrap();
    println!("Part 1: {}", computer.output_buf.back().unwrap());

    computer.init(&input).unwrap();
    computer.add_input(5);
    task::block_on(computer.run()).unwrap();
    println!("Part 2: {}", computer.output_buf.back().unwrap());
}
