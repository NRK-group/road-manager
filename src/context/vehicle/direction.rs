use super::Vehicle;
use rand::Rng;
pub use sdl2::pixels::Color;
pub use std::cell::RefCell;
pub struct Direction {
    pub left: Vec<RefCell<Vehicle>>,
    pub right: Vec<RefCell<Vehicle>>,
    pub straight: Vec<RefCell<Vehicle>>,
}
impl Direction {
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
            straight: Vec::new(),
        }
    }
    pub fn remove_first_from_direction(
        &mut self,
        vehicle_direction: VehicleDirection,
    ) -> RefCell<Vehicle> {
        match vehicle_direction {
            VehicleDirection::Left => self.left.remove(0),
            VehicleDirection::Straight => self.straight.remove(0),
            VehicleDirection::Right => self.right.remove(0),
        }
    }
    pub fn get_len_of_direction(&self, vehicle_direction: &VehicleDirection) -> usize {
        match vehicle_direction {
            VehicleDirection::Left => self.left.len(),
            VehicleDirection::Straight => self.straight.len(),
            VehicleDirection::Right => self.right.len(),
        }
    }
}
#[derive(Clone)]
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
    pub fn push_to_vehicle_direction(self, direction: &mut Direction, vehicle: RefCell<Vehicle>) {
        match self {
            VehicleDirection::Left => direction.left.push(vehicle),
            VehicleDirection::Straight => direction.straight.push(vehicle),
            VehicleDirection::Right => direction.right.push(vehicle),
        }
    }
}
