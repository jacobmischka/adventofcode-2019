use rayon::prelude::*;

use std::{io, iter};

type Num = i32;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let input = get_nums(line.trim());

    let mut nums = input.clone();
    for _ in 0..100 {
        nums = run_phase(&nums);
    }

    let part_1: String = nums.iter().take(8).map(|x| x.to_string()).collect();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", do_part_2(&line));
}

fn do_part_2(s: &str) -> String {
    let input = get_nums(s);
    let mut digits = input.iter().take(7).collect::<Vec<&Num>>();

    digits.reverse();

    let offset = digits
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (*x as u32 * 10u32.pow(i as _))) as usize;

    let mut nums: Vec<Num> = input
        .iter()
        .cycle()
        .take(10000 * input.len())
        .copied()
        .collect();

    for _ in 0..100 {
        nums = run_phase(&nums);
    }

    nums.iter()
        .skip(offset)
        .take(8)
        .map(|x| x.to_string())
        .collect()
}

fn get_nums(s: &str) -> Vec<Num> {
    s.chars().map(|x| x.to_digit(10).unwrap() as _).collect()
}

fn run_phase(signal: &[Num]) -> Vec<Num> {
    let len = signal.len();

    (0..len)
        .into_par_iter()
        .map(|i| {
            signal
                .iter()
                .zip(get_pattern(i))
                .filter(|(_, y)| **y != 0)
                .fold(0, |acc, (x, y)| acc + (x * y))
                .abs()
                % 10
        })
        .collect()
}

const BASE_PATTERN: [Num; 4] = [0, 1, 0, -1];

fn get_pattern(output_element: usize) -> impl Iterator<Item = &'static Num> {
    let mut repeats = BASE_PATTERN
        .iter()
        .map(|i| iter::repeat(i).take(output_element + 1));

    repeats
        .next()
        .unwrap()
        .chain(repeats.next().unwrap())
        .chain(repeats.next().unwrap())
        .chain(repeats.next().unwrap())
        .cycle()
        .skip(1)
}

#[test]
fn examples_work() {
    let mut ex1 = get_nums("80871224585914546619083218645595");
    let mut ex2 = get_nums("19617804207202209144916044189917");

    for _ in 0..100 {
        ex1 = run_phase(&ex1);
        ex2 = run_phase(&ex2);
    }

    assert_eq!(
        ex1.iter().take(8).copied().collect::<Vec<Num>>(),
        vec![2, 4, 1, 7, 6, 1, 7, 6]
    );
    assert_eq!(
        ex2.iter().take(8).copied().collect::<Vec<Num>>(),
        vec![7, 3, 7, 4, 5, 4, 1, 8]
    );

    assert_eq!(do_part_2("03036732577212944063491565474664"), "84462026");
    assert_eq!(do_part_2("02935109699940807407585447034323"), "78725270");
    assert_eq!(do_part_2("03081770884921959731165446850517"), "53553731");
}

#[test]
fn phase_works() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8];

    nums = run_phase(&nums);
    assert_eq!(nums, vec![4, 8, 2, 2, 6, 1, 5, 8]);

    nums = run_phase(&nums);
    assert_eq!(nums, vec![3, 4, 0, 4, 0, 4, 3, 8]);

    nums = run_phase(&nums);
    assert_eq!(nums, vec![0, 3, 4, 1, 5, 5, 1, 8]);

    nums = run_phase(&nums);
    assert_eq!(nums, vec![0, 1, 0, 2, 9, 4, 9, 8]);
}

#[test]
fn get_pattern_works() {
    assert_eq!(
        get_pattern(0).take(8).copied().collect::<Vec<Num>>(),
        vec![1, 0, -1, 0, 1, 0, -1, 0]
    );
    assert_eq!(
        get_pattern(7).take(8).copied().collect::<Vec<Num>>(),
        vec![0, 0, 0, 0, 0, 0, 0, 1]
    );
}
