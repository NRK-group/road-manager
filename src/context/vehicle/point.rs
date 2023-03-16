use crate::origin::*;
pub use std::ops::Add;

use super::direction::VehicleDirection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

impl Add<Point> for Point {
    type Output = Self;
    /// Returns the sum of two points.
    ///
    /// # Arguments
    ///
    /// * `self` - The first point.
    /// * `rhs` - The second point.
    ///
    /// # Returns
    ///
    /// The sum of two points.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::point::Point;
    ///
    /// let point = Point(1, 2) + Point(3, 4);
    ///
    /// assert_eq!(point, Point(4, 6));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Point {
    /// Returns a new point.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin of the point.
    /// * `direction` - The direction of the point.
    ///
    /// # Returns
    ///
    /// A new point depends on the origin and vehicle direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::origin::Origin;
    /// use context::vehicle::direction::VehicleDirection;
    /// use context::vehicle::point::Point;
    ///
    /// let point = Point::new(Origin::North, VehicleDirection::Left);
    ///
    /// assert_eq!(point, Point(270, -40));
    /// 
    /// let point = Point::new(Origin::South, VehicleDirection::Left);
    /// 
    /// assert_eq!(point, Point(310, 610));
    /// 
    /// let point = Point::new(Origin::West, VehicleDirection::Left);
    /// 
    /// assert_eq!(point, Point(-40, 310));
    /// 
    /// let point = Point::new(Origin::East, VehicleDirection::Left);
    /// 
    /// assert_eq!(point, Point(610, 270));
    /// ```
    pub fn new(origin: Origin, direction: VehicleDirection) -> Self {
        match origin {
            Origin::North => match direction {
                VehicleDirection::Left => Self(270, -40),
                VehicleDirection::Straight => Self(230, -40),
                VehicleDirection::Right => Self(190, -40),
            },
            Origin::South => match direction {
                VehicleDirection::Left => Self(310, 610),
                VehicleDirection::Straight => Self(350, 610),
                VehicleDirection::Right => Self(390, 610),
            },
            Origin::West => match direction {
                VehicleDirection::Left => Self(-40, 310),
                VehicleDirection::Straight => Self(-40, 350),
                VehicleDirection::Right => Self(-40, 390),
            },
            Origin::East => match direction {
                VehicleDirection::Left => Self(610, 270),
                VehicleDirection::Straight => Self(610, 230),
                VehicleDirection::Right => Self(610, 190),
            },
        }
    }
}
