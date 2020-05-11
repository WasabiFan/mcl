use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl ops::Add<(f64, f64)> for Point {
    type Output = Point;
    fn add(self, rhs: (f64, f64)) -> Self::Output {
        Point { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

impl ops::AddAssign<(f64, f64)> for Point {
    fn add_assign(&mut self, rhs: (f64, f64)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}