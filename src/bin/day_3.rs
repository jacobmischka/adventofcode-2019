use std::io;
use std::ops::Add;

fn main() {
    let mut line1 = String::new();
    let mut line2 = String::new();

    let _ = io::stdin().read_line(&mut line1);
    let _ = io::stdin().read_line(&mut line2);

    let wire1 = Wire::new(&line1).unwrap();
    let wire2 = Wire::new(&line2).unwrap();
}

#[derive(Debug)]
struct Wire {
    points: Vec<Point>,
}

impl Wire {
    fn new(path: &str) -> Result<Self, Error> {
        let mut pos = Point(0, 0);
        Ok(Wire {
            points: path
                .trim()
                .split(',')
                .map(|step| {
                    pos = pos + Step::new(step)?;
                    Ok(pos)
                })
                .collect::<Result<Vec<Point>, Error>>()?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);

impl Add<Step> for Point {
    type Output = Self;

    fn add(self, step: Step) -> Self {
        match step.direction {
            Direction::Up => Point(self.0, self.1 + step.distance as i32),
            Direction::Down => Point(self.0, self.1 - step.distance as i32),
            Direction::Left => Point(self.0 - step.distance as i32, self.1),
            Direction::Right => Point(self.0 + step.distance as i32, self.1),
        }
    }
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    distance: u32,
}

#[derive(Debug)]
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
