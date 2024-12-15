use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Index<char> for Point {
    type Output = isize;

    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' => &self.x,
            'y' => &self.y,
            i => panic!("Index {i} not in Point"),
        }
    }
}

impl IndexMut<char> for Point {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'x' => &mut self.x,
            'y' => &mut self.y,
            i => panic!("Index {i} not in Point"),
        }
    }
}

impl Point {
    pub fn in_map(&self, x_dim: isize, y_dim: isize) -> bool {
        (0..x_dim).contains(&self.x) && (0..y_dim).contains(&self.y)
    }

    pub fn hemming_distance(&self, rhs: Self) -> Self {
        Self {
            x: (self.x - rhs.x).abs(),
            y: (self.y - rhs.y).abs(),
        }
    }

    pub fn turn_counterclockwise(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn turn_clockwise(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn wrapping_add(self, rhs: Self, limits: Self) -> Self {
        Self {
            x: (self.x + rhs.x).rem_euclid(limits.x),
            y: (self.y + rhs.y).rem_euclid(limits.y),
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<&Self> for &Point {
    type Output = Point;

    fn add(self, rhs: &Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<isize> for &Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Point {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl std::ops::Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl std::ops::Div<isize> for Point {
    type Output = Self;

    fn div(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl From<Point> for (isize, isize) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
