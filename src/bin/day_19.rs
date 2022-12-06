use adventofcode_2019::{get_input, intcode_computer::IntcodeComputer};
use async_std::{prelude::*, task};

fn main() {
    let program = get_input().unwrap();
    task::block_on(run(program.trim()));
}

async fn run(program: &str) {
    let (inputs, outputs) = IntcodeComputer::create_io();
    let mut computer = IntcodeComputer::new(&inputs.1, &outputs.0);
    computer.init(program).unwrap();

    let result = computer
        .repeat()
        .race(async {
            let mut result = 0;
            for y in 0..50 {
                for x in 0..50 {
                    inputs.0.send(x).await.unwrap();
                    inputs.0.send(y).await.unwrap();
                    let output = outputs.1.recv().await.unwrap();
                    result += output;
                    print!(
                        "{}",
                        match output {
                            1 => "#",
                            0 => ".",
                            _ => "?",
                        }
                    );
                }
                println!();
            }

            Ok(result)
        })
        .await
        .unwrap();

    println!("Part 1: {result}");

    computer.init(program).unwrap();

    let result = computer
        .repeat()
        .race(async {
            let mut result = (0, 0);
            let mut prev_start = 0;
            'y_loop: for y in 0..10000 {
                let mut start = 0;
                let mut end = 0;
                'x_loop: for x in prev_start..10000 {
                    inputs.0.send(x).await.unwrap();
                    inputs.0.send(y).await.unwrap();
                    let output = outputs.1.recv().await.unwrap();

                    if output == 1 && start == 0 {
                        start = x;
                    }
                    if output == 0 && end == 0 && start != 0 {
                        end = x;
                        break 'x_loop;
                    }
                }
                prev_start = start;
                if (end - start) >= 100 {
                    for x in start..(end - 99) {
                        if {
                            inputs.0.send(x).await.unwrap();
                            inputs.0.send(y + 99).await.unwrap();
                            outputs.1.recv().await.unwrap()
                        } == 1
                            && {
                                inputs.0.send(x + 99).await.unwrap();
                                inputs.0.send(y + 99).await.unwrap();
                                outputs.1.recv().await.unwrap()
                            } == 1
                        {
                            result = (x, y);
                            break 'y_loop;
                        }
                    }
                }
            }

            for y in (result.1 - 10)..(result.1 + 110) {
                for x in (result.0 - 10)..(result.0 + 110) {
                    inputs.0.send(x).await.unwrap();
                    inputs.0.send(y).await.unwrap();
                    let output = outputs.1.recv().await.unwrap();
                    if x >= result.0
                        && x <= result.0 + 99
                        && y >= result.1
                        && y <= result.1 + 99
                        && output == 1
                    {
                        print!("O");
                    } else {
                        print!(
                            "{}",
                            match output {
                                1 => "#",
                                0 => ".",
                                _ => "?",
                            }
                        );
                    }
                }

                println!();
            }

            Ok(result.0 * 10000 + result.1)
        })
        .await
        .unwrap();

    println!("Part 2: {result}");
}
