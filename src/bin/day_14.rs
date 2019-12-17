use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::ops::Mul;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let reactions: HashMap<String, Reaction> = stdin
        .lock()
        .lines()
        .filter_map(|line| Reaction::from_str(&line.unwrap()).ok())
        .map(|r| (r.output.unit.clone(), r))
        .collect();

    let fuel_reaction = reactions.get("FUEL").unwrap();
    let mut stockpile = HashMap::new();
    println!(
        "Part 1: {}",
        fuel_reaction.get_ore_requirements(&reactions, &mut stockpile)
    );
}

#[derive(Debug, Clone)]
struct Reaction {
    output: Measurement,
    inputs: Vec<Measurement>,
}

impl Reaction {
    fn get_ore_requirements(
        &self,
        reactions: &HashMap<String, Reaction>,
        stockpile: &mut HashMap<String, u32>,
    ) -> u32 {
        let mut amount = 0;

        for input in self.inputs.iter() {
            if &input.unit == "ORE" {
                amount += input.amount;
            } else {
                let reaction = reactions.get(&input.unit).unwrap();
                let in_stockpile = stockpile.entry(input.unit.to_string()).or_default();
                let used_from_stockpile = input.amount.min(*in_stockpile);
                *in_stockpile -= used_from_stockpile;

                let num_needed = input.amount - used_from_stockpile;
                let reactions_needed =
                    (num_needed as f32 / reaction.output.amount as f32).ceil() as u32;
                amount += reaction.get_ore_requirements(reactions, stockpile) * reactions_needed;
            }
        }

        amount
    }
}

impl FromStr for Reaction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MEASUREMENT_RE: Regex = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
        }

        let mut inputs: Vec<Measurement> = MEASUREMENT_RE
            .captures_iter(s)
            .map(|cap| {
                Ok(Measurement {
                    unit: cap[2].to_string(),
                    amount: cap[1]
                        .parse()
                        .map_err(|_| Error::InvalidInputError(cap[1].to_string()))?,
                })
            })
            .collect::<Result<Vec<Measurement>, Error>>()?;

        let output = inputs.pop().expect("no output");

        Ok(Reaction { output, inputs })
    }
}

#[derive(Debug, Clone)]
struct Measurement {
    unit: String,
    amount: u32,
}

impl Mul<u32> for Measurement {
    type Output = Self;

    fn mul(mut self, rhs: u32) -> Self {
        self.amount *= rhs;
        self
    }
}

#[derive(Debug)]
enum Error {
    InvalidInputError(String),
}
