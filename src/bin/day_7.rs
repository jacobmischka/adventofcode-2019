use async_std::task;
use futures::future::join_all;
use permute::permute;

use adventofcode_2019::intcode_computer::*;

use std::io;

const NUM_AMPS: usize = 5;

fn main() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();

    println!("Part 1: {}", get_max_output(&input, 0));
}

fn get_max_output(program: &str, initial_input: Int) -> Int {
    let mut amps: [IntcodeComputer; NUM_AMPS] = [
        IntcodeComputer::new(),
        IntcodeComputer::new(),
        IntcodeComputer::new(),
        IntcodeComputer::new(),
        IntcodeComputer::new(),
    ];

    let phase_options: Vec<Int> = (0..NUM_AMPS).map(|x| x as _).collect();

    let mut max = 0;

    for permutation in permute(phase_options) {
        reset_amps(&mut amps, program);
        let output = task::block_on(run_amps(&mut amps, &permutation, initial_input));
        max = max.max(output);
    }

    max
}

fn reset_amps(amps: &mut [IntcodeComputer], program: &str) {
    for amp in amps.iter_mut() {
        amp.init(program).unwrap();
    }
}

async fn run_amps(amps: &mut [IntcodeComputer], phase_settings: &[Int], initial_input: Int) -> Int {
    let len = amps.len();
    amps[0].add_input(initial_input);

    for (amp, phase_setting) in amps.iter().zip(phase_settings) {
        amp.add_input(*phase_setting);
    }

    let mut tasks = Vec::new();

    for i in 0..len {
        tasks.push(poll(
            &mut amps[i],
            &mut amps[if i == 0 { len - 1 } else { i - 1 }],
        ));
    }

    join_all(tasks).await;

    amps[len - 1].get_output().await
}

async fn poll(amp: &mut IntcodeComputer, input_amp: &mut IntcodeComputer) {
    let f = amp.run();
    loop {
        let state = amp.state();
        if state == &OperationState::Exited {
            break;
        }

        let input = input_amp.get_output().await;
        amp.add_input(input);
    }
    f.await;
}

#[test]
fn examples_work() {
    assert_eq!(
        get_max_output("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 0),
        43210
    );
    assert_eq!(
        get_max_output(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
            0
        ),
        54321
    );
    assert_eq!(get_max_output("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", 0), 65210);
}
