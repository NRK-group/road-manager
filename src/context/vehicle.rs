pub mod point;
pub use point::*;
pub mod direction;
pub use direction::*;
pub mod origin;
pub use origin::*;
pub mod queue;
pub use queue::*;
pub struct Vehicle {
    pub direction: VehicleDirection,
    pub origin: Origin,
    pub point: Point,
}

impl Vehicle {
    pub fn new(origin: Origin) -> Self {
        let direction = VehicleDirection::random();
        Self {
            origin: origin.clone(),
            direction: direction.clone(),
            point: Point::new(origin, direction),
        }
    }
}

pub enum VehicleType {
    Horizontal,
    Verticle
}
