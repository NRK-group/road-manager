use crate::vehicle::{Origin, Vehicle};

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
    pub fn push_vehicle(&mut self, origin: Origin, id: i32) {
        let example = Vehicle::new(origin.clone(), id);
        match origin {
            Origin::East => self.east.add_vehicle_to_queue(example),
            Origin::West => self.west.add_vehicle_to_queue(example),
            Origin::North => self.north.add_vehicle_to_queue(example),
            Origin::South =>self.south.add_vehicle_to_queue(example),
        }
    }

}


pub struct TurningQueue {
    pub north: TurningDirection,
    pub east: TurningDirection,
    pub west: TurningDirection,
    pub south: TurningDirection,
}

impl TurningQueue {
    pub fn new() -> Self {
        Self {
            north: TurningDirection::new(),
            east: TurningDirection::new(),
            west: TurningDirection::new(),
            south: TurningDirection::new(),
        }
    }
}

pub struct TurningDirection {
    pub left: bool,
    pub straight: bool,
    pub right: bool,
}

impl TurningDirection {
    pub fn new() -> Self {
        Self { left: false, straight: false, right: false }
    }
}