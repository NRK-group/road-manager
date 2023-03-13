use crate::vehicle::{Origin, Vehicle};
use std::cell::RefCell;

use super::Direction;
pub struct Queue {
    pub north: Direction,
    pub east: Direction,
    pub south: Direction,
    pub west: Direction,
}


impl Queue {
    pub fn new() -> Self {
        Self {
            north: Direction::new(),
            east: Direction::new(),
            south: Direction::new(),
            west: Direction::new(),
        }
    }
    pub fn push_vehicle(&mut self, origin: Origin) {
        let example = Vehicle::new(origin.clone());
        match origin {
            Origin::East => self.east.add_vehicle_to_queue(example),
            Origin::West => self.west.add_vehicle_to_queue(example),
            Origin::North => self.north.add_vehicle_to_queue(example),
            Origin::South =>self.south.add_vehicle_to_queue(example),
        }
    }
}
