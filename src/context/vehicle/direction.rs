use super::{origin::*, Vehicle};
use rand::Rng;
pub use sdl2::pixels::Color;
use std::cell::RefCell;
pub struct Direction {
    left: Vec<RefCell<Vehicle>>,
    right: Vec<RefCell<Vehicle>>,
    straight: Vec<RefCell<Vehicle>>,
}
impl Direction {
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
            straight: Vec::new(),
        }
    }

    pub fn add_vehicle_to_queue(&mut self, vehicle: Vehicle) {
        match vehicle.direction {
            VehicleDirection::Left => self.left.push(RefCell::new(vehicle)),
            VehicleDirection::Straight => self.straight.push(RefCell::new(vehicle)),
            VehicleDirection::Right => self.right.push(RefCell::new(vehicle)),
        };
    }
}
pub enum VehicleDirection {
    Left,
    Straight,
    Right,
}

impl VehicleDirection {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(1..=3) {
            1 => Self::Left,
            2 => Self::Right,
            _ => Self::Straight,
        }
    }
}
