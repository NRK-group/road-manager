use super::Direction;
use crate::vehicle::{Origin, Vehicle, VehicleDirection};
use std::cell::{Ref, RefCell};
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
        if self.check_c_q(&origin, &vehicle_direction) {
            match origin {
                Origin::East => {
                    vehicle_direction.push_to_vehicle_direction(&mut self.east, vehicle)
                }
                Origin::West => {
                    vehicle_direction.push_to_vehicle_direction(&mut self.west, vehicle)
                }
                Origin::North => {
                    vehicle_direction.push_to_vehicle_direction(&mut self.north, vehicle)
                }
                Origin::South => {
                    vehicle_direction.push_to_vehicle_direction(&mut self.south, vehicle)
                }
            }
        }
    }
    pub fn remove_first_in_queue(
        &mut self,
        origin: Origin,
        vehicle_direction: VehicleDirection,
    ) -> RefCell<Vehicle> {
        match origin {
            Origin::East => self.east.remove_first_from_direction(vehicle_direction),
            Origin::West => self.west.remove_first_from_direction(vehicle_direction),
            Origin::North => self.north.remove_first_from_direction(vehicle_direction),
            Origin::South => self.south.remove_first_from_direction(vehicle_direction),
        }
    }
    pub fn check_c_q(&self, origin: &Origin, vehicle_direction: &VehicleDirection) -> bool {
        match origin {
            Origin::East => self.check_lane(&self.east, |val| val.point.0 < 560, vehicle_direction),
            Origin::West => self.check_lane(&self.west, |val| val.point.0 > 10, vehicle_direction),
            Origin::North => {
                self.check_lane(&self.north, |val| val.point.1 > 20, vehicle_direction)
            }
            Origin::South => {
                self.check_lane(&self.south, |val| val.point.1 < 560, vehicle_direction)
            }
        }
    }
    fn check_lane<F>(
        &self,
        lane: &Direction,
        condition: F,
        vehicle_direction: &VehicleDirection,
    ) -> bool
    where
        F: Fn(&Ref<Vehicle>) -> bool,
    {
        match vehicle_direction {
            VehicleDirection::Left => lane
                .left
                .last()
                .map_or(true, |val| condition(&val.borrow())),
            VehicleDirection::Straight => lane
                .straight
                .last()
                .map_or(true, |val| condition(&val.borrow())),
            VehicleDirection::Right => lane
                .right
                .last()
                .map_or(true, |val| condition(&val.borrow())),
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
