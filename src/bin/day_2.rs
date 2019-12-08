use async_std::task;

use adventofcode_2019::intcode_computer::*;

use std::io;

const EXPECTED_OUTPUT: Int = 19690720;

const MAX_INPUT: Int = 99;

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();

    let mut computer = IntcodeComputer::new();

    {
        computer.init(&input).unwrap();
        computer.write(1, 12);
        computer.write(2, 2);
        task::block_on(computer.run()).unwrap();

        println!("Part 1: {}", computer.read(0));
    }

    let mut noun = 0;
    let mut verb = 0;

    loop {
        computer.init(&input).unwrap();
        computer.write(1, noun);
        computer.write(2, verb);
        task::block_on(computer.run()).unwrap();

        if computer.read(0) == EXPECTED_OUTPUT {
            break;
        } else {
            if verb == MAX_INPUT {
                if noun == MAX_INPUT {
                    break;
                } else {
                    verb = 0;
                    noun += 1;
                }
            } else {
                verb += 1;
            }
        }
    }

    println!("Part 2: {}", 100 * noun + verb);
}
