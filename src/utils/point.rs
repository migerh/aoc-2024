use std::{
    fmt::{Display, Formatter, Result},
    hash::Hash,
    ops::{Add, Neg, Sub},
};

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&format!("({}, {})", self.x, self.y))
    }
}

impl<T: Clone> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point<isize> {
    pub fn neighbors(&self) -> [Self; 4] {
        [
            self.clone() + Point::from((-1, 0)),
            self.clone() + Point::from((0, 1)),
            self.clone() + Point::from((1, 0)),
            self.clone() + Point::from((0, -1)),
        ]
    }
}

impl Point<i32> {
    pub fn neighbors(&self) -> [Self; 4] {
        [
            self.clone() + Point::from((-1, 0)),
            self.clone() + Point::from((0, 1)),
            self.clone() + Point::from((1, 0)),
            self.clone() + Point::from((0, -1)),
        ]
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(value: Point<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T: Add> Add for Point<T> {
    type Output = Point<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub> Sub for Point<T> {
    type Output = Point<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Neg> Neg for Point<T> {
    type Output = Point<T::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}
