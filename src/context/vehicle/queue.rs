use super::Direction;
use crate::vehicle::{Origin, Vehicle, VehicleDirection};
use std::cell::RefCell;
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
    pub fn create_vehicle(&mut self, origin: Origin, id: i32) {
        let vehicle_direction = VehicleDirection::random();
        let vehicle = RefCell::new(Vehicle::new(origin.clone(), &vehicle_direction, id));
        match origin {
            Origin::East => vehicle_direction.push_to_vehicle_direction(&mut self.east, vehicle),
            Origin::West => vehicle_direction.push_to_vehicle_direction(&mut self.east, vehicle),
            Origin::North => vehicle_direction.push_to_vehicle_direction(&mut self.east, vehicle),
            Origin::South => vehicle_direction.push_to_vehicle_direction(&mut self.east, vehicle),
        }
    }
    pub fn remove_first_in_queue(
        &mut self,
        origin: Origin,
        direction: VehicleDirection,
    ) -> RefCell<Vehicle> {
        match origin {
            Origin::East => match direction {
                VehicleDirection::Left => self.east.left.remove(0),
                VehicleDirection::Straight => self.east.straight.remove(0),
                VehicleDirection::Right => self.east.right.remove(0),
            },
            Origin::West => match direction {
                VehicleDirection::Left => self.west.left.remove(0),
                VehicleDirection::Straight => self.west.straight.remove(0),
                VehicleDirection::Right => self.west.right.remove(0),
            },

            Origin::North => match direction {
                VehicleDirection::Left => self.north.left.remove(0),
                VehicleDirection::Straight => self.north.straight.remove(0),
                VehicleDirection::Right => self.north.right.remove(0),
            },

            Origin::South => match direction {
                VehicleDirection::Left => self.south.left.remove(0),
                VehicleDirection::Straight => self.south.straight.remove(0),
                VehicleDirection::Right => self.south.right.remove(0),
            },
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
        Self {
            left: false,
            straight: false,
            right: false,
        }
    }
}
