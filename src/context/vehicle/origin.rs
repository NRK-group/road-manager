use super::{Queue, Vehicle, VehicleDirection};
use rand::Rng;
use std::cell::RefCell;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Origin {
    North,
    East,
    South,
    West,
}

impl Origin {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(1..=4) {
            1 => Self::North,
            2 => Self::South,
            3 => Self::West,
            _ => Self::East,
        }
    }
    pub fn add_vehicle_to_origin(
        &self,
        vehicle_direction: &VehicleDirection,
        queue: &mut Queue,
        vehicle: RefCell<Vehicle>,
    ) {
        match self {
            Origin::East => vehicle_direction.push_to_vehicle_direction(&mut queue.east, vehicle),
            Origin::West => vehicle_direction.push_to_vehicle_direction(&mut queue.west, vehicle),
            Origin::North => vehicle_direction.push_to_vehicle_direction(&mut queue.north, vehicle),
            Origin::South => vehicle_direction.push_to_vehicle_direction(&mut queue.south, vehicle),
        }
    }
    pub fn get_len_of_queue_from_direction(
        &self,
        queue: &Queue,
        vehicle_direction: &VehicleDirection,
    ) -> usize {
        match self {
            Origin::East => vehicle_direction.get_len_of_vehicle_direction(&queue.east),
            Origin::West => vehicle_direction.get_len_of_vehicle_direction(&queue.west),
            Origin::North => vehicle_direction.get_len_of_vehicle_direction(&queue.north),
            Origin::South => vehicle_direction.get_len_of_vehicle_direction(&queue.south),
        }
    }
}
