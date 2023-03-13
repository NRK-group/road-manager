use crate::origin::*;
pub use std::ops::Add;

use super::direction::VehicleDirection;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

impl Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Point {
    pub fn new(origin: Origin, direction: VehicleDirection) -> Self {
        match origin {
            Origin::North => match direction {
                VehicleDirection::Left => Self(270, 0),
                VehicleDirection::Straight => Self(230, 0),
                VehicleDirection::Right => Self(190, 0),
            },
            Origin::South => match direction {
                VehicleDirection::Left => Self(310, 610),
                VehicleDirection::Straight => Self(350, 610),
                VehicleDirection::Right => Self(390, 610),
            },
            Origin::West => match direction {
                VehicleDirection::Left => Self(0, 270),
                VehicleDirection::Straight => Self(0, 230),
                VehicleDirection::Right => Self(0, 190),
            },
            Origin::East => match direction {
                VehicleDirection::Left => Self(610, 310),
                VehicleDirection::Straight => Self(610, 350),
                VehicleDirection::Right => Self(610, 390),
            },
        }
    }
}
