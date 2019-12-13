use std::collections::HashSet;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord(u32, u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vector(i32, i32);

impl Vector {
    fn new(x: i32, y: i32) -> Self {
        let gcf = gcd(x.abs() as _, y.abs() as _) as i32;

        Vector(x / gcf, y / gcf)
    }
}

impl Sub for &Coord {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        let x = self.0 as i32 - other.0 as i32;
        let y = self.1 as i32 - other.1 as i32;

        Vector::new(x, y)
    }
}

fn main() {
    let mut coords: Vec<Coord> = Vec::new();
    let mut max = Coord(0, 0);
    for (x, line) in io::stdin().lock().lines().enumerate() {
        for (y, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                coords.push(Coord(x as _, y as _));
            }
            max = Coord(x as _, y as _);
        }
    }

    let mut max_detectable = 0;
    let mut index: Option<usize> = None;
    for (i, p) in coords.iter().enumerate() {
        let d = count_detectable(p, coords.as_slice());
        if d > max_detectable {
            max_detectable = d;
            index = Some(i);
        }
    }

    let monitoring_station = coords.remove(index.unwrap());

    println!("Part 1: {}", max_detectable);
}

fn count_detectable(coord: &Coord, coords: &[Coord]) -> u32 {
    let mut set: HashSet<Vector> = HashSet::new();

    for other_coord in coords {
        if coord != other_coord {
            set.insert(other_coord - coord);
        }
    }

    set.len() as _
}

fn vaporize(laser: &Coord, asteroids: Vec<Coord>) -> Vec<Coord> {
    let mut vaporized = Vec::new();

    vaporized
}

fn rotate_clockwise(current: &Coord, max: &Coord) -> Coord {
    let Coord(x, y) = current;
    let x = *x;
    let y = *y;
    if y == 0 {
        if x < max.0 {
            Coord(x + 1, y)
        } else {
            Coord(x, y + 1)
        }
    } else if x == max.0 {
        if y < max.1 {
            Coord(x, y + 1)
        } else {
            Coord(x - 1, y)
        }
    } else if y == max.1 {
        if x > 0 {
            Coord(x - 1, y)
        } else {
            Coord(x, y - 1)
        }
    } else {
        // x == 0
        if y > 0 {
            Coord(x, y - 1)
        } else {
            Coord(x + 1, y)
        }
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else if b > a {
        gcd(b, a)
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn gcd_works() {
    assert_eq!(gcd(48, 18), 6);
}
