use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::ops::{Add, AddAssign};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let reactions: HashMap<String, Reaction> = stdin
        .lock()
        .lines()
        .filter_map(|line| Reaction::from_str(&line.unwrap()).ok())
        .map(|r| (r.output.unit.clone(), r))
        .collect();

    dbg!(&reactions);
}

#[derive(Debug, Clone)]
struct Reaction {
    output: Measurement,
    inputs: Vec<Measurement>,
}

impl FromStr for Reaction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MEASUREMENT_RE: Regex = Regex::new(r"(\d+) (\s+)").unwrap();
        }

        let mut inputs: Vec<Measurement> = MEASUREMENT_RE
            .captures_iter(s)
            .map(|cap| Measurement {
                unit: cap[1].to_string(),
                amount: cap[0].parse().unwrap(),
            })
            .collect();

        let output = inputs.pop().expect("no output");

        Ok(Reaction { output, inputs })
    }
}

#[derive(Debug, Clone)]
struct Measurement {
    unit: String,
    amount: u32,
}

#[derive(Debug)]
enum Error {
    InvalidInputError,
}
