use async_std::sync::channel;
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

    let phase_options: Vec<Int> = (0..NUM_AMPS).map(|x| x as _).collect();
    println!("Part 1: {}", get_max_output(&input, &phase_options, 0));

    let phase_options: Vec<Int> = (5..(5 + NUM_AMPS)).map(|x| x as _).collect();
    println!("Part 2: {}", get_max_output(&input, &phase_options, 0));
}

fn get_max_output(program: &str, phase_options: &Vec<Int>, initial_input: Int) -> Int {
    let mut max = 0;

    for permutation in permute(phase_options.clone()) {
        let output = task::block_on(run_amps(&permutation, program, initial_input));
        max = max.max(output);
    }

    max
}

async fn run_amps(phase_settings: &[Int], program: &str, initial_input: Int) -> Int {
    let a_io = channel(BUFFER_SIZE);
    let b_io = channel(BUFFER_SIZE);
    let c_io = channel(BUFFER_SIZE);
    let d_io = channel(BUFFER_SIZE);
    let e_io = channel(BUFFER_SIZE);

    let mut a = IntcodeComputer::new(&a_io.1, &b_io.0);
    let mut b = IntcodeComputer::new(&b_io.1, &c_io.0);
    let mut c = IntcodeComputer::new(&c_io.1, &d_io.0);
    let mut d = IntcodeComputer::new(&d_io.1, &e_io.0);
    let mut e = IntcodeComputer::new(&e_io.1, &a_io.0);

    (a_io.0).send(phase_settings[0]).await;
    (b_io.0).send(phase_settings[1]).await;
    (c_io.0).send(phase_settings[2]).await;
    (d_io.0).send(phase_settings[3]).await;
    (e_io.0).send(phase_settings[4]).await;

    (a_io.0).send(initial_input).await;

    let mut amps = vec![&mut a, &mut b, &mut c, &mut d, &mut e];
    for amp in amps.iter_mut() {
        amp.init(program).unwrap();
    }
    let runnings = amps.iter_mut().map(|a| a.run());
    join_all(runnings).await;

    a_io.1.recv().await.unwrap()
}

#[test]
fn examples_work() {
    let phase_options: Vec<Int> = (0..NUM_AMPS).map(|x| x as _).collect();
    assert_eq!(
        get_max_output(
            "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
            &phase_options,
            0
        ),
        43210
    );
    assert_eq!(
        get_max_output(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
            &phase_options,
            0
        ),
        54321
    );
    assert_eq!(get_max_output("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", &phase_options, 0), 65210);

    let phase_options: Vec<Int> = (5..(5 + NUM_AMPS)).map(|x| x as _).collect();
    assert_eq!(
        get_max_output(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
            &phase_options,
            0
        ),
        139629729
    );
    assert_eq!(
        get_max_output(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
            &phase_options,
            0
        ),
        18216
    );
}
