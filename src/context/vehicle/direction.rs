use super::Vehicle;
use rand::Rng;
pub use sdl2::pixels::Color;
pub use std::cell::RefCell;
#[derive(Debug)]
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
        vehicle_direction: &VehicleDirection,
    ) -> RefCell<Vehicle> {
        match vehicle_direction {
            VehicleDirection::Left => self.left.remove(0),
            VehicleDirection::Straight => self.straight.remove(0),
            VehicleDirection::Right => self.right.remove(0),
        }
    }
    pub fn remove_out_of_bounds_vehicles(&mut self) {
        //Check each lane
        if let Some(vehicle) = self.right.first() {
            let points = vehicle.borrow().point;
            if points.1 > 650 || points.1 < -40 || points.0 < -40 || points.0 > 650 {
                self.right.remove(0);
            }
        };
        if let Some(vehicle) = self.straight.first() {
            let points = vehicle.borrow().point;
            if points.1 > 650 || points.1 < -40 || points.0 < -40 || points.0 > 650 {
                self.straight.remove(0);
            }
        };
        if let Some(vehicle) = self.left.first() {
            let points = vehicle.borrow().point;
            if points.1 > 650 || points.1 < -40 || points.0 < -40 || points.0 > 650 {
                self.left.remove(0);
            }
        };
    }
}
#[derive(Debug, Clone)]
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
    pub fn push_to_vehicle_direction(&self, direction: &mut Direction, vehicle: RefCell<Vehicle>) {
        match self {
            VehicleDirection::Left => direction.left.push(vehicle),
            VehicleDirection::Straight => direction.straight.push(vehicle),
            VehicleDirection::Right => direction.right.push(vehicle),
        }
    }
    pub fn get_len_of_vehicle_direction(&self, direction: &Direction) -> usize {
        match self {
            VehicleDirection::Left => direction.left.len(),
            VehicleDirection::Straight => direction.straight.len(),
            VehicleDirection::Right => direction.right.len(),
        }
    }
}
