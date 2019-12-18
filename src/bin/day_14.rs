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
    let single_consumption = fuel_reaction.produce(1, &reactions, &mut stockpile);

    println!("Part 1: {}", single_consumption);

    let mut ore_remaining = ORE_TOTAL - single_consumption;

    let mut states: HashMap<String, (u64, u64)> = HashMap::new();
    states.insert(
        serialize_stockpile(&stockpile),
        (*stockpile.get("FUEL").unwrap(), ore_remaining),
    );

    let mut cycle_start: Option<(u64, u64)> = None;
    let mut cycle_end: Option<(u64, u64)> = None;

    let mut states_backup = states.clone();
    let mut stockpile_backup = stockpile.clone();
    let mut remaining_backup = ore_remaining;

    let mut count = 10000;

    loop {
        let consumed = fuel_reaction.produce(count, &reactions, &mut stockpile);
        if let Some(remaining) = ore_remaining.checked_sub(consumed) {
            ore_remaining = remaining;
            if ore_remaining == 0 {
                break;
            }
            states_backup = states.clone();
            stockpile_backup = stockpile.clone();
            remaining_backup = ore_remaining;
            let counts = (*stockpile.get("FUEL").unwrap(), ore_remaining);

            if let Some(start) = states.insert(serialize_stockpile(&stockpile), counts) {
                if start != counts {
                    cycle_start = Some(start);
                    cycle_end = Some(counts);
                }
                break;
            }
        } else {
            states = states_backup.clone();
            stockpile = stockpile_backup.clone();
            ore_remaining = remaining_backup;
            count /= 2;
        }
    }

    if ore_remaining > 0 {
        if let (Some(cycle_start), Some(cycle_end)) = (cycle_start, cycle_end) {
            let cycle_steps = cycle_end.0 - cycle_start.0;
            let cycle_consumption = cycle_start.1 - cycle_end.1;

            let num_cycles = ore_remaining / cycle_consumption;

            ore_remaining -= cycle_consumption * num_cycles;

            let fuel_created = stockpile.get_mut("FUEL").unwrap();
            *fuel_created += cycle_steps * num_cycles;

            while ore_remaining > 0 {
                if let Some(remaining) =
                    ore_remaining.checked_sub(fuel_reaction.produce(1, &reactions, &mut stockpile))
                {
                    ore_remaining = remaining;
                } else {
                    break;
                }
            }

            if ore_remaining > 0 {
                *stockpile.get_mut("FUEL").unwrap() -= 1;
            }
        }
    }

    let fuel_created = *stockpile.get("FUEL").unwrap();

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
        count: u64,
        reactions: &HashMap<String, Reaction>,
        stockpile: &mut Stockpile,
    ) -> u64 {
        let mut ore_consumed = 0;

        let produced = self.output.amount * count;

        for input in self.inputs.iter() {
            let mut num_needed = input.amount * count;

            if &input.unit == "ORE" {
                ore_consumed += num_needed;
            } else {
                let reaction = reactions.get(&input.unit).unwrap();

                let in_stockpile = stockpile.entry(input.unit.to_string()).or_default();

                let used_from_stockpile = num_needed.min(*in_stockpile);
                *in_stockpile -= used_from_stockpile;
                num_needed -= used_from_stockpile;

                let reactions_needed =
                    (num_needed as f64 / reaction.output.amount as f64).ceil() as u64;

                ore_consumed += reaction.produce(reactions_needed, reactions, stockpile);

                *stockpile.entry(input.unit.to_string()).or_default() -= num_needed;
            }
        }

        *stockpile.entry(self.output.unit.to_string()).or_default() += produced;

        ore_consumed
    }
}

type Stockpile = HashMap<String, u64>;

fn serialize_stockpile(stockpile: &Stockpile) -> String {
    stockpile
        .iter()
        .filter_map(|(k, v)| {
            if k == "FUEL" {
                None
            } else {
                Some(format!("{}:{}", k, v))
            }
        })
        .collect::<Vec<String>>()
        .join(",")
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
