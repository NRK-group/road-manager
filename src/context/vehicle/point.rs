use crate::origin::*;
pub use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

impl Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Point {
    pub fn new(origin: Origin) -> Self {
        match origin {
            Origin::North => Self(250, -50),
            Origin::South => Self(300, 600),
            Origin::East => Self(600, 250),
            Origin::West => Self(-50, 300),
        }
    }
}
