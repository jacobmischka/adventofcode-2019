use std::io;
use std::num;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut pieces = input.trim().split('-');
    let min = pieces.next().unwrap();
    let max = pieces.next().unwrap();

    count_valid_passwords(min, max).unwrap();
}

fn count_valid_passwords(min_s: &str, max_s: &str) -> Result<(), Error> {
    let min: u32 = min_s.parse()?;
    let max: u32 = max_s.parse()?;

    let mut part1 = 0;
    let mut part2 = 0;
    let mut x = min;

    while x <= max {
        if password_valid(&x.to_string(), true) {
            part1 += 1;
        }
        if password_valid(&x.to_string(), false) {
            part2 += 1;
        }
        x += 1;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn password_valid(password: &str, double_can_be_in_group: bool) -> bool {
    let mut has_double = false;

    let mut double_start = 0;
    let mut double_end = 0;

    if password.len() != 6 {
        return false;
    }

    let len = password.len();
    let chars: Vec<char> = password.chars().collect();

    for i in 1..len {
        let digit: u32 = chars[i].to_digit(10).unwrap();
        let prev_digit: u32 = chars[i - 1].to_digit(10).unwrap();

        if digit < prev_digit {
            return false;
        }

        if digit == prev_digit {
            if double_can_be_in_group {
                has_double = true;
            } else {
                if double_end != i - 1 {
                    if double_end - double_start == 1 {
                        has_double = true;
                    }

                    double_start = i - 1;
                }

                double_end = i;
            }
        }
    }

    if !double_can_be_in_group && double_end - double_start == 1 {
        has_double = true;
    }

    has_double
}

#[derive(Debug)]
enum Error {
    NonNumberError(num::ParseIntError),
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self {
        Error::NonNumberError(error)
    }
}

#[test]
fn examples_work() {
    assert_eq!(password_valid("111111", true), true);
    assert_eq!(password_valid("223450", true), false);
    assert_eq!(password_valid("123789", true), false);

    assert_eq!(password_valid("112233", false), true);
    assert_eq!(password_valid("123444", false), false);
    assert_eq!(password_valid("111122", false), true);
}
