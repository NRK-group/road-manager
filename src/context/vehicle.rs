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
}

pub enum VehicleType {
    Horizontal,
    Verticle,
}
