use std::cmp;
use std::io;
use std::ops::Add;

fn main() {
    let mut line1 = String::new();
    let mut line2 = String::new();

    let _ = io::stdin().read_line(&mut line1);
    let _ = io::stdin().read_line(&mut line2);

    let wire1 = Wire::new(line1.trim()).unwrap();
    let wire2 = Wire::new(line2.trim()).unwrap();

    println!(
        "Part 1: {}",
        get_closest_intersection_distance(&wire1, &wire2).unwrap()
    );
    println!(
        "Part 2: {}",
        get_fewest_intersection_steps(&wire1, &wire2).unwrap()
    );
}

fn get_closest_intersection_distance(wire1: &Wire, wire2: &Wire) -> Option<u32> {
    let intersections = wire1.intersections(&wire2);

    let mut min_distance = None;
    let origin = Point::new(0, 0);

    for intersection in intersections.iter().skip(1) {
        let distance = origin.manhattan_distance(&intersection);
        match min_distance {
            Some(old_min) => {
                if distance < old_min {
                    min_distance = Some(distance);
                }
            }
            None => min_distance = Some(distance),
        }
    }

    min_distance
}

fn get_fewest_intersection_steps(wire1: &Wire, wire2: &Wire) -> Option<u32> {
    let intersections = wire1.intersections_and_steps(&wire2);

    let mut min_steps = None;

    for (intersection, (paths1, paths2)) in intersections.iter().skip(1) {
        let steps = get_paths_length(paths1)
            + paths1
                .last()
                .unwrap()
                .end()
                .manhattan_distance(&intersection)
            + get_paths_length(paths2)
            + paths2
                .last()
                .unwrap()
                .end()
                .manhattan_distance(&intersection);
        match min_steps {
            Some(old_min) => {
                if steps < old_min {
                    min_steps = Some(steps)
                }
            }
            None => min_steps = Some(steps),
        }
    }

    min_steps
}

fn get_paths_length(paths: &[Path]) -> u32 {
    paths.iter().fold(0, |acc, x| acc + x.step.distance)
}

#[derive(Debug)]
struct Wire {
    paths: Vec<Path>,
}

impl Wire {
    fn new(path: &str) -> Result<Self, Error> {
        let mut pos = Point::new(0, 0);
        Ok(Wire {
            paths: path
                .trim()
                .split(',')
                .map(|step_s| {
                    let step = Step::new(step_s)?;
                    let path = Path { start: pos, step };
                    pos = pos + step;

                    Ok(path)
                })
                .collect::<Result<Vec<Path>, Error>>()?,
        })
    }

    fn intersections(&self, other: &Wire) -> Vec<Point> {
        self.intersections_and_steps(&other)
            .into_iter()
            .map(|(p, _)| p)
            .collect()
    }

    fn intersections_and_steps<'a, 'b>(
        &'a self,
        other: &'b Wire,
    ) -> Vec<(Point, (&'a [Path], &'b [Path]))> {
        let mut intersections = Vec::new();

        for (i1, p1) in self.paths.iter().enumerate() {
            for (i2, p2) in other.paths.iter().enumerate() {
                if let Some(intersection) = p1.intersection(&p2) {
                    intersections.push((intersection, (&self.paths[0..i1], &other.paths[0..i2])));
                }
            }
        }

        intersections
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn manhattan_distance(&self, other: &Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

#[derive(Debug, Clone, Copy)]
struct Path {
    start: Point,
    step: Step,
}

impl Path {
    fn end(&self) -> Point {
        self.start + self.step
    }

    fn endpoints(&self) -> (Point, Point) {
        use Direction::*;

        match self.step.direction {
            Up => (self.start, self.end()),
            Down => (self.end(), self.start),
            Right => (self.start, self.end()),
            Left => (self.end(), self.start),
        }
    }

    fn intersection(&self, other: &Path) -> Option<Point> {
        use Direction::*;

        if let Some(((hori_min, hori_max), (vert_min, vert_max))) = match self.step.direction {
            Up | Down => match other.step.direction {
                Up | Down => None,
                Left | Right => Some((other.endpoints(), self.endpoints())),
            },
            Left | Right => match other.step.direction {
                Up | Down => Some((self.endpoints(), other.endpoints())),
                Left | Right => None,
            },
        } {
            if hori_min.x <= vert_min.x
                && hori_max.x >= vert_max.x
                && vert_min.y <= hori_min.y
                && vert_max.y >= hori_max.y
            {
                Some(Point::new(vert_min.x, hori_min.y))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Add<Step> for Point {
    type Output = Self;

    fn add(self, step: Step) -> Self {
        match step.direction {
            Direction::Up => Point::new(self.x, self.y + step.distance as i32),
            Direction::Down => Point::new(self.x, self.y - step.distance as i32),
            Direction::Left => Point::new(self.x - step.distance as i32, self.y),
            Direction::Right => Point::new(self.x + step.distance as i32, self.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Step {
    direction: Direction,
    distance: u32,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Step {
    fn new(s: &str) -> Result<Self, Error> {
        let (direction_s, count_s) = s.split_at(1);
        let distance: u32 = count_s
            .parse()
            .map_err(|_| Error::InvalidStepError(s.to_string()))?;
        let direction = match direction_s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(Error::InvalidStepError(s.to_string())),
        }?;

        Ok(Step {
            direction,
            distance,
        })
    }
}

#[derive(Debug)]
enum Error {
    InvalidStepError(String),
}

#[test]
fn intersection_works() {
    let p1 = Path {
        start: Point::new(0, 0),
        step: Step {
            direction: Direction::Right,
            distance: 10,
        },
    };
    let p2 = Path {
        start: Point::new(5, 10),
        step: Step {
            direction: Direction::Down,
            distance: 20,
        },
    };

    assert_eq!(p1.intersection(&p2), Some(Point::new(5, 0)));

    let p1 = Path {
        start: Point::new(-10, 33),
        step: Step {
            direction: Direction::Left,
            distance: 10,
        },
    };
    let p2 = Path {
        start: Point::new(-13, -100),
        step: Step {
            direction: Direction::Up,
            distance: 201,
        },
    };

    assert_eq!(p1.intersection(&p2), Some(Point::new(-13, 33)));
}

#[test]
fn examples_work() {
    let ex1 = (
        Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap(),
        Wire::new("U62,R66,U55,R34,D71,R55,D58,R83").unwrap(),
    );
    assert_eq!(get_closest_intersection_distance(&ex1.0, &ex1.1), Some(159));
    assert_eq!(get_fewest_intersection_steps(&ex1.0, &ex1.1), Some(610));

    let ex2 = (
        Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap(),
        Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap(),
    );

    assert_eq!(get_closest_intersection_distance(&ex2.0, &ex2.1), Some(135));
    assert_eq!(get_fewest_intersection_steps(&ex2.0, &ex2.1), Some(410));
}
