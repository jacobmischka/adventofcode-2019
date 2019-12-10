use async_std::task;

use adventofcode_2019::intcode_computer::*;

use std::io;

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();

    let mut computer = IntcodeComputer::new();

    computer.init(&input).unwrap();
    let output = task::block_on(async {
        computer.add_input(1).await;
        computer.run().await.unwrap();
        let mut ret = -1;
        while let Some(output) = computer.get_output().await {
            ret = output
        }
        ret
    });
    println!("Part 1: {}", output);

    computer.init(&input).unwrap();
    let output = task::block_on(async {
        computer.add_input(5).await;
        computer.run().await.unwrap();
        let mut ret = -1;
        while let Some(output) = computer.get_output().await {
            ret = output
        }
        ret
    });

    println!("Part 2: {}", output);
}
