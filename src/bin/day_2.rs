use adventofcode_2019::intcode_computer::*;

use std::io;

const EXPECTED_OUTPUT: u32 = 19690720;

const MAX_INPUT: u32 = 99;

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();

    {
        let mut program = IntcodeProgram::new(&input).unwrap();
        program.write(1, 12);
        program.write(2, 2);
        program.run().unwrap();

        println!("Part 1: {}", program.read(0));
    }

    let mut noun = 0;
    let mut verb = 0;

    loop {
        let mut program = IntcodeProgram::new(&input).unwrap();
        program.write(1, noun);
        program.write(2, verb);
        program.run().unwrap();

        if program.read(0) == EXPECTED_OUTPUT {
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
