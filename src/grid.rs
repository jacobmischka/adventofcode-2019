use rustbox::{Color, RustBox, RB_NORMAL};

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Coord(pub i64, pub i64);

#[derive(Debug, Clone)]
pub struct Grid<T>(HashMap<Coord, T>);

pub struct GridItemVec<T>(VecDeque<T>);

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn bounds(&self) -> ((i64, i64), (i64, i64)) {
        let min_x = self.keys().fold(std::i64::MAX, |acc, c| acc.min(c.0));
        let max_x = self.keys().fold(std::i64::MIN, |acc, c| acc.max(c.0));
        let min_y = self.keys().fold(std::i64::MAX, |acc, c| acc.min(c.1));
        let max_y = self.keys().fold(std::i64::MIN, |acc, c| acc.max(c.1));

        ((min_x, min_y), (max_x, max_y))
    }

    pub fn make_positive(&mut self) {
        let mut map = HashMap::new();
        let ((min_x, min_y), _) = self.bounds();
        let x_diff = 0 - min_x;
        let y_diff = 0 - min_y;

        if x_diff > 0 || y_diff > 0 {
            for (mut c, item) in self.drain() {
                c.0 += x_diff;
                c.1 += y_diff;
                map.insert(c, item);
            }
        }

        self.0 = map;
    }
}

pub trait Fillable<T: Clone> {
    fn fill(&mut self, x_bounds: (i64, i64), y_bounds: (i64, i64), item: T);
}

impl<T: Clone> Fillable<T> for Grid<T> {
    fn fill(&mut self, (x_min, x_max): (i64, i64), (y_min, y_max): (i64, i64), item: T) {
        for x in x_min..x_max {
            for y in y_min..y_max {
                self.entry(Coord(x, y)).or_insert(item.clone());
            }
        }
    }
}

pub trait Drawable {
    fn draw(&self, rustbox: &RustBox);
}

/// This will panic if any coordinates are negative.
/// Ensure they're positive by calling make_positive() first.
impl<T> Drawable for Grid<T>
where
    T: fmt::Display,
{
    fn draw(&self, rustbox: &RustBox) {
        for (Coord(x, y), item) in self.iter() {
            rustbox.print(
                *x as _,
                *y as _,
                RB_NORMAL,
                Color::White,
                Color::Black,
                &item.to_string(),
            );
        }
    }
}

impl<T> Deref for Grid<T> {
    type Target = HashMap<Coord, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Deref for GridItemVec<T> {
    type Target = VecDeque<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for GridItemVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ((min_x, min_y), (max_x, max_y)) = self.bounds();

        let default = T::default();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let item = match self.get(&Coord(x, y)) {
                    Some(item) => item,
                    _ => &default,
                };
                write!(f, "{}", item)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> fmt::Display for GridItemVec<T>
where
    T: fmt::Display + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let default = T::default();

        write!(
            f,
            "{}",
            match self.front() {
                Some(item) => item,
                _ => &default,
            }
        )
    }
}
