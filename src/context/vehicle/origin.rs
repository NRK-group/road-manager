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
    /// Returns a random origin.
    ///
    /// # Returns
    ///
    /// A random origin.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::origin::Origin;
    ///
    /// let origin = Origin::random();
    /// ```
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(1..=4) {
            1 => Self::North,
            2 => Self::South,
            3 => Self::West,
            _ => Self::East,
        }
    }
    /// Pushes a vehicle to the queue in the specified direction.
    ///
    /// # Arguments
    ///
    /// * `vehicle_direction` - The direction of the vehicles in the queue.
    /// * `queue` - The queue to push the vehicle to.
    /// * `vehicle` - The vehicle to push to the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::origin::Origin;
    /// use context::vehicle::queue::Queue;
    /// use context::vehicle::vehicle_direction::VehicleDirection;
    /// use context::vehicle::vehicle::Vehicle;
    /// use std::cell::RefCell;
    ///
    /// let origin = Origin::North;
    /// let mut queue = Queue::new();
    /// let vehicle_direction = VehicleDirection::North;
    /// let vehicle = RefCell::new(Vehicle::new());
    ///     
    /// origin.add_vehicle_to_origin(&vehicle_direction, &mut queue, vehicle);
    /// ```
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
    /// Returns the length of the queue in the specified direction.
    ///
    /// # Arguments
    ///
    /// * `queue` - The queue to return the length of.
    /// * `vehicle_direction` - The direction of the vehicles in the queue.
    ///
    /// # Returns
    ///
    /// The length of the queue in the specified direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::origin::Origin;
    /// use context::vehicle::queue::Queue;
    /// use context::vehicle::vehicle_direction::VehicleDirection;
    ///
    /// let origin = Origin::North;
    /// let queue = Queue::new();
    /// let vehicle_direction = VehicleDirection::North;
    ///
    /// let len = origin.get_len_of_queue_from_direction(&queue, &vehicle_direction);
    /// ```
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
