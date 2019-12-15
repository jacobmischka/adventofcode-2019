use lazy_static::lazy_static;
use regex::Regex;

use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::ops::{Add, AddAssign};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let moons: Vec<RefCell<Moon>> = stdin
        .lock()
        .lines()
        .filter_map(|line| Moon::from_str(&line.unwrap()).map(|m| RefCell::new(m)).ok())
        .collect();

    let p1_moons = moons.clone();
    for _ in 0..1000 {
        simulate_universe(p1_moons.as_slice());
    }

    let total_energy = p1_moons
        .iter()
        .fold(0, |acc, moon| acc + moon.borrow().energy());

    println!("Part 1: {}", total_energy);

    // Part 2 is not finished. While this works for the simple example, it is not efficient enough
    // for the second example or actual inputs.

    let mut states: HashSet<Vec<Moon>> = HashSet::new();

    while states.insert(moons.iter().map(|moon| moon.borrow().clone()).collect()) {
        simulate_universe(moons.as_slice());
    }

    println!("Part 2: {}", states.len());
}

fn simulate_universe(moons: &[RefCell<Moon>]) {
    for moon in moons.iter() {
        for other_moon in moons.iter() {
            if other_moon != moon {
                moon.borrow_mut().apply_gravity(&other_moon.borrow());
            }
        }
    }

    for moon in moons.iter() {
        moon.borrow_mut().apply_velocity();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    position: AxesTriple,
    velocity: AxesTriple,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Moon {
            position: AxesTriple { x, y, z },
            velocity: AxesTriple::default(),
        }
    }

    fn apply_gravity(&mut self, other: &Moon) {
        if self.position.x < other.position.x {
            self.velocity.x += 1;
        } else if self.position.x > other.position.x {
            self.velocity.x -= 1;
        }

        if self.position.y < other.position.y {
            self.velocity.y += 1;
        } else if self.position.y > other.position.y {
            self.velocity.y -= 1;
        }

        if self.position.z < other.position.z {
            self.velocity.z += 1;
        } else if self.position.z > other.position.z {
            self.velocity.z -= 1;
        }
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    fn potential_energy(&self) -> u32 {
        self.position.x.abs() as u32 + self.position.y.abs() as u32 + self.position.z.abs() as u32
    }

    fn kinetic_energy(&self) -> u32 {
        self.velocity.x.abs() as u32 + self.velocity.y.abs() as u32 + self.velocity.z.abs() as u32
    }

    fn energy(&self) -> u32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

impl FromStr for Moon {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"<x=(?P<x>.+), y=(?P<y>.+), z=(?P<z>.+)>").unwrap();
        }

        if let Some(caps) = RE.captures(s) {
            Ok(Moon::new(
                caps.name("x").unwrap().as_str().parse().unwrap(),
                caps.name("y").unwrap().as_str().parse().unwrap(),
                caps.name("z").unwrap().as_str().parse().unwrap(),
            ))
        } else {
            Err(Error::InvalidInputError)
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct AxesTriple {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for AxesTriple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for AxesTriple {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug)]
enum Error {
    InvalidInputError,
}
