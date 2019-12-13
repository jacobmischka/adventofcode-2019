use std::collections::HashSet;
use std::convert::From;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord(i32, i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path(i32, i32);

impl Path {
    fn len(&self) -> f64 {
        (((self.0.abs() as u32).pow(2) + (self.1.abs() as u32).pow(2)) as f64).sqrt()
    }
}

#[test]
fn len_works() {
    assert_eq!(Path(1, 1).len(), 2f64.sqrt());
    assert_eq!(Path(3, 4).len(), 5f64);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vector(i32, i32);

impl Vector {
    fn new(x: i32, y: i32) -> Self {
        let gcf = gcd(x.abs() as _, y.abs() as _) as i32;

        Vector(x / gcf, y / gcf)
    }
}

impl From<&Path> for Vector {
    fn from(path: &Path) -> Self {
        Self::new(path.0, path.1)
    }
}

impl PartialEq<Path> for Vector {
    fn eq(&self, other: &Path) -> bool {
        let other_vec: Vector = other.into();
        self == &other_vec
    }
}

#[test]
fn vector_path_junk_works() {
    assert_eq!(Vector::new(10, 10), Path(10, 10));
    assert_eq!(Vector::new(1, 1), Path(10, 10));
}

impl Sub for &Coord {
    type Output = Path;

    fn sub(self, other: Self) -> Path {
        let x = self.0 as i32 - other.0 as i32;
        let y = self.1 as i32 - other.1 as i32;

        Path(x, y)
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

    let vaporized = vaporize(&monitoring_station, coords);
    let two_hundredth = &vaporized[199];
    println!("Part 2: {}", two_hundredth.0 * 100 + two_hundredth.1);
}

fn count_detectable(coord: &Coord, coords: &[Coord]) -> u32 {
    let mut set: HashSet<Vector> = HashSet::new();

    for other_coord in coords {
        if coord != other_coord {
            set.insert((&(other_coord - coord)).into());
        }
    }

    set.len() as _
}

fn vaporize(laser: &Coord, mut asteroids: Vec<Coord>) -> Vec<Coord> {
    let mut vaporized = Vec::new();
    let radius = 2000u32;

    let mut target = Coord(laser.0, laser.1 - radius as i32);

    loop {
        let laser_vector: Vector = (&(&target - &laser)).into();

        let mut min_distance = std::f64::MAX;
        let mut asteroid_index_to_vaporize: Option<usize> = None;

        for (i, asteroid) in asteroids.iter().enumerate() {
            let a_path: Path = asteroid - laser;
            if laser_vector == a_path {
                let len = a_path.len();
                if a_path.len() < min_distance {
                    asteroid_index_to_vaporize = Some(i);
                    min_distance = len;
                }
            }
        }

        if let Some(i) = asteroid_index_to_vaporize {
            vaporized.push(asteroids.remove(i));
            dbg!(vaporized.len(), asteroids.len());
        }

        if asteroids.is_empty() {
            break;
        }

        target = rotate_clockwise(&target, &laser, radius);
    }

    vaporized
}

fn rotate_clockwise(current: &Coord, center: &Coord, radius: u32) -> Coord {
    let Coord(x, y) = current;
    let x = *x;
    let y = *y;
    let min = Coord(center.0 - radius as i32, center.1 - radius as i32);
    let max = Coord(center.0 + radius as i32, center.1 + radius as i32);
    if y == min.1 {
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
        if x > min.0 {
            Coord(x - 1, y)
        } else {
            Coord(x, y - 1)
        }
    } else {
        // x == min.0
        if y > min.1 {
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
