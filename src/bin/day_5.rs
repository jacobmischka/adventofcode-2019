use async_std::task;

use adventofcode_2019::intcode_computer::*;

use std::io;

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();

    let (inputs, outputs) = IntcodeComputer::create_io();
    let mut computer = IntcodeComputer::new(&inputs.1, &outputs.0);

    computer.init(&input).unwrap();
    let output = task::block_on(async {
        (inputs.0).send(1).await.unwrap();
        computer.run().await.unwrap();
        let mut ret = -1;
        while let Ok(output) = (outputs.1).recv().await {
            ret = output
        }
        ret
    });
    println!("Part 1: {}", output);

    computer.init(&input).unwrap();
    let output = task::block_on(async {
        (inputs.0).send(5).await.unwrap();
        computer.run().await.unwrap();
        let mut ret = -1;
        while let Ok(output) = (outputs.1).recv().await {
            ret = output
        }
        ret
    });

    println!("Part 2: {}", output);
}
