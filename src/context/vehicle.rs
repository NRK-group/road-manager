pub mod point;
pub use point::*;
pub mod direction;
pub use direction::*;
pub mod origin;
pub use origin::*;
pub mod queue;
pub use queue::*;
pub struct Vehicle {
    pub direction: Direction,
    pub origin: Origin,
    pub color: Color,
    pub point: Point,
}

impl Vehicle {
    pub fn new(origin: Origin) -> Self {
        let (direction, color) = Direction::random();
        Self {
            origin: origin.clone(),
            direction: direction.clone(),
            color,
            point: Point::new(origin.clone()),
        }
    }
}
