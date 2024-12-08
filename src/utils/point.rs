#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
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

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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
