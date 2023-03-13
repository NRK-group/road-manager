use crate::vehicle::{Origin, Vehicle};
use std::cell::RefCell;
pub struct Queue {
    pub north: Vec<RefCell<Vehicle>>,
    pub east: Vec<RefCell<Vehicle>>,
    pub south: Vec<RefCell<Vehicle>>,
    pub west: Vec<RefCell<Vehicle>>,
}
impl Queue {
    pub fn new() -> Self {
        Self {
            north: Vec::new(),
            east: Vec::new(),
            south: Vec::new(),
            west: Vec::new(),
        }
    }
    pub fn push_vehicle(&mut self, origin: Origin) {
        let example = RefCell::new(Vehicle::new(origin.clone()));
        match origin {
            Origin::East => self.east.push(example),
            Origin::West => self.west.push(example),
            Origin::North => self.north.push(example),
            Origin::South => self.south.push(example),
        }
    }
}
