pub mod point;
pub use point::*;
pub mod direction;
pub use direction::*;
pub mod origin;
pub use origin::*;
pub mod queue;
pub use queue::*;
#[derive(Debug)]
pub struct Vehicle {
    pub id: i32,
    pub velocity: i32,
    pub direction: VehicleDirection,
    pub origin: Origin,
    pub point: Point,
}

impl Vehicle {
    pub fn new(origin: Origin, direction: &VehicleDirection, id: i32) -> Self {
        Self {
            id,
            velocity: 2,
            origin: origin.clone(),
            direction: direction.clone(),
            point: Point::new(origin, direction.clone()),
        }
    }

    pub fn turn(&self) -> bool {
        match self.origin {
            Origin::North => match self.direction {
                VehicleDirection::Left => self.point.1 >= 300 ,
                VehicleDirection::Straight => self.point.1 >= 380,
                VehicleDirection::Right => self.point.1 >= 180,
            },
            Origin::East => match self.direction {
                VehicleDirection::Left => self.point.0 <= 270 ,
                VehicleDirection::Straight => self.point.0 <= 220,
                VehicleDirection::Right => self.point.0 <= 390,
            } ,
            Origin::West => match self.direction {
                VehicleDirection::Left => self.point.0 >= 300,
                VehicleDirection::Straight => self.point.0 >= 340,
                VehicleDirection::Right => self.point.0 >= 180,
            } ,
            Origin::South => match self.direction {
                VehicleDirection::Left => self.point.1 <= 270 ,
                VehicleDirection::Straight => self.point.1 <= 220,
                VehicleDirection::Right => self.point.1 <= 390,
            },

        }
    }
}

pub enum VehicleType {
    Horizontal,
    Verticle,
}
