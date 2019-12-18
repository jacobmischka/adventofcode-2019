use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::ops::Mul;
use std::str::FromStr;

const ORE_TOTAL: u64 = 1_000_000_000_000;

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
    let single_consumption = fuel_reaction.produce(&reactions, &mut stockpile);

    println!("Part 1: {}", single_consumption);

    let mut ore_remaining = ORE_TOTAL - single_consumption;

    let mut after_one = stockpile.clone();
    after_one.remove("FUEL");

    let mut cycle_consumption = 0;
    loop {
        let consumed = fuel_reaction.produce(&reactions, &mut stockpile);
        ore_remaining -= consumed;
        cycle_consumption += consumed;

        if after_one
            .iter()
            .all(|(k, v)| stockpile.get(k).unwrap() == v)
        {
            break;
        }
    }

    let num_cycles = ore_remaining / cycle_consumption;

    ore_remaining -= cycle_consumption * num_cycles;

    let fuel_created = stockpile.get_mut("FUEL").unwrap();
    *fuel_created += (*fuel_created - 1) * num_cycles;

    while ore_remaining > 0 {
        if let Some(remaining) =
            ore_remaining.checked_sub(fuel_reaction.produce(&reactions, &mut stockpile))
        {
            ore_remaining = remaining;
        } else {
            break;
        }
    }

    let mut fuel_created = *stockpile.get("FUEL").unwrap();
    if ore_remaining > 0 {
        fuel_created -= 1;
    }

    println!("Part 2: {}", fuel_created);
}

#[derive(Debug, Clone)]
struct Reaction {
    output: Measurement,
    inputs: Vec<Measurement>,
}

impl Reaction {
    fn produce(
        &self,
        reactions: &HashMap<String, Reaction>,
        stockpile: &mut HashMap<String, u64>,
    ) -> u64 {
        let mut ore_consumed = 0;

        for input in self.inputs.iter() {
            if &input.unit == "ORE" {
                ore_consumed += input.amount;
            } else {
                let reaction = reactions.get(&input.unit).unwrap();

                let in_stockpile = stockpile.entry(input.unit.to_string()).or_default();
                let used_from_stockpile = input.amount.min(*in_stockpile);
                *in_stockpile -= used_from_stockpile;

                let num_needed = input.amount - used_from_stockpile;
                let reactions_needed =
                    (num_needed as f32 / reaction.output.amount as f32).ceil() as usize;

                for _ in 0..reactions_needed {
                    ore_consumed += reaction.produce(reactions, stockpile);
                }

                *stockpile.entry(input.unit.to_string()).or_default() -= num_needed;
            }
        }

        *stockpile.entry(self.output.unit.to_string()).or_default() += self.output.amount;

        ore_consumed
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
    amount: u64,
}

impl Mul<u64> for Measurement {
    type Output = Self;

    fn mul(mut self, rhs: u64) -> Self {
        self.amount *= rhs;
        self
    }
}

#[derive(Debug)]
enum Error {
    InvalidInputError(String),
}
