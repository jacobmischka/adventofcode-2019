use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::ops::Sub;

use std::f64::consts::{FRAC_PI_2, PI};

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

type Angle = f64;

impl From<&Vector> for Angle {
    fn from(v: &Vector) -> Self {
        if v.1 == 0 {
            if v.0 > 0 {
                FRAC_PI_2
            } else {
                PI + FRAC_PI_2
            }
        } else {
            let angle = (-1.0 * v.0 as f64 / v.1 as f64).atan();
            if v.1 > 0 {
                angle + PI
            } else if angle < 0.0 {
                angle + PI + PI
            } else {
                angle
            }
        }
    }
}

#[test]
fn angle_vector_works() {
    use std::f64::consts::FRAC_PI_4;

    assert_eq!(Angle::from(&Vector::new(0, -1)), 0.0);
    assert_eq!(Angle::from(&Vector::new(1, -1)), FRAC_PI_4);
    assert_eq!(Angle::from(&Vector::new(1, 0)), FRAC_PI_2);
    assert_eq!(Angle::from(&Vector::new(1, 1)), FRAC_PI_2 + FRAC_PI_4);
    assert_eq!(Angle::from(&Vector::new(0, 1)), PI);
    assert_eq!(Angle::from(&Vector::new(-1, 1)), PI + FRAC_PI_4);
    assert_eq!(Angle::from(&Vector::new(-1, 0)), PI + FRAC_PI_2);
    assert_eq!(
        Angle::from(&Vector::new(-1, -1)),
        PI + FRAC_PI_2 + FRAC_PI_4
    );
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
    for (y, line) in io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                coords.push(Coord(x as _, y as _));
            }
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

    let vaporized = vaporize(&monitoring_station, coords.as_slice());
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

fn vaporize<'a, 'b>(laser: &'a Coord, asteroids: &'b [Coord]) -> Vec<&'b Coord> {
    let mut vaporized = Vec::new();

    let mut angle_map: HashMap<String, Vec<(usize, Path)>> = HashMap::new();
    let mut angles: Vec<Angle> = Vec::new();

    for (i, asteroid) in asteroids.iter().enumerate() {
        let a_path: Path = asteroid - laser;
        let angle = Angle::from(&Vector::from(&a_path));
        angle_map
            .entry(format!("{:0.*}", 10, angle))
            .or_default()
            .push((i, a_path));
        angles.push(angle);
    }

    let angles: HashSet<String> = angles.iter().map(|a| format!("{:0.*}", 10, a)).collect();
    let mut angles: Vec<String> = angles.into_iter().collect();
    angles.sort_unstable_by(|a, b| a.cmp(&b));

    for (_, v) in angle_map.iter_mut() {
        v.sort_unstable_by(|a, b| a.1.len().partial_cmp(&b.1.len()).unwrap());
    }

    while vaporized.len() < asteroids.len() {
        for angle in angles.iter() {
            let v = angle_map.get_mut(angle).unwrap();
            if !v.is_empty() {
                let (i, _) = v.remove(0);
                vaporized.push(&asteroids[i]);
            }
        }
    }

    vaporized
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
