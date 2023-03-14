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
    pub fn add_vehicle_to_queue_with_refcell(&mut self, vehicle: RefCell<Vehicle>) {
        let direction = vehicle.borrow().direction.clone();
        direction.push_to_vehicle_direction(self, vehicle)
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
