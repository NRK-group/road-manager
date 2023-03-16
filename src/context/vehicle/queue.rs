use super::Direction;
use crate::vehicle::{Origin, Vehicle, VehicleDirection};
use std::cell::{Ref, RefCell};
#[derive(Debug)]
pub struct Queue {
    pub north: Direction,
    pub east: Direction,
    pub south: Direction,
    pub west: Direction,
}

impl Queue {
    /// Returns a new queue.
    ///
    /// # Returns
    ///
    /// A new queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::queue::Queue;
    ///
    /// let queue = Queue::new();
    /// ```
    pub fn new() -> Self {
        Self {
            north: Direction::new(),
            east: Direction::new(),
            south: Direction::new(),
            west: Direction::new(),
        }
    }
    /// Creates a vehicle and adds it to the queue.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin of the vehicle.
    /// * `id` - The id of the vehicle.
    /// * `vehicle_direction` - The direction of the vehicle.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::origin::Origin;
    /// use context::vehicle::queue::Queue;
    /// use context::vehicle::vehicle_direction::VehicleDirection;
    ///
    /// let mut queue = Queue::new();
    ///
    /// queue.create_vehicle(Origin::North, 1, VehicleDirection::North);
    /// ```
    pub fn create_vehicle(&mut self, origin: Origin, id: i32, vehicle_direction: VehicleDirection) {
        let vehicle = RefCell::new(Vehicle::new(origin.clone(), &vehicle_direction, id));
        origin.add_vehicle_to_origin(&vehicle_direction, self, vehicle)
    }
    /// Remove and Returns the first vehicle in the specific queue.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin of the vehicle.
    /// * `vehicle_direction` - The direction of the vehicle.
    ///
    /// # Returns
    ///
    /// The first vehicle in the queue.
    ///
    /// # Examples
    /// ```
    /// use context::vehicle::origin::Origin;
    /// use context::vehicle::queue::Queue;
    /// use context::vehicle::vehicle_direction::VehicleDirection;
    ///
    /// let mut queue = Queue::new();
    ///
    /// queue.create_vehicle(Origin::North, 1, VehicleDirection::North);
    ///
    /// let vehicle = queue.remove_first_in_queue(&Origin::North, &VehicleDirection::North);
    /// ```
    pub fn remove_first_in_queue(
        &mut self,
        origin: &Origin,
        vehicle_direction: &VehicleDirection,
    ) -> RefCell<Vehicle> {
        match origin {
            Origin::East => self.east.remove_first_from_direction(&vehicle_direction),
            Origin::West => self.west.remove_first_from_direction(&vehicle_direction),
            Origin::North => self.north.remove_first_from_direction(&vehicle_direction),
            Origin::South => self.south.remove_first_from_direction(&vehicle_direction),
        }
    }
    /// Returns bool if the vehicle is safe distance from the last vehicle in specific orgin and direction.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin of the vehicle.
    /// * `vehicle_direction` - The direction of the vehicle.
    ///
    /// # Returns
    ///
    /// Bool if the vehicle is safe distance from the last vehicle.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::origin::Origin;
    /// use context::vehicle::queue::Queue;
    /// use context::vehicle::vehicle_direction::VehicleDirection;
    ///
    /// let mut queue = Queue::new();
    ///
    /// queue.create_vehicle(Origin::North, 1, VehicleDirection::North);
    ///
    /// let is_safe_distance = queue.is_safe_distance_from_last_vehicle(&Origin::North, &VehicleDirection::North);
    ///
    /// assert_eq!(is_safe_distance, true);
    /// ```
    pub fn is_safe_distance_from_last_vehicle(
        &self,
        origin: &Origin,
        vehicle_direction: &VehicleDirection,
    ) -> bool {
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
    /// Returns bool if the vehicle is safe distance from the last vehicle
    /// 
    /// # Arguments
    /// 
    /// * `lane` - The lane of the vehicle.
    /// * `condition` - The condition of the vehicle.
    /// * `vehicle_direction` - The direction of the vehicle.
    /// 
    /// # Returns
    /// 
    /// Bool if the vehicle is safe distance from the last vehicle.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use context::vehicle::direction::Direction;
    /// use context::vehicle::queue::Queue;
    /// use context::vehicle::vehicle_direction::VehicleDirection;
    /// 
    /// let mut queue = Queue::new();
    /// 
    /// let is_safe_distance = queue.check_lane(&Direction::new(), |val| val.point.0 < 560, &VehicleDirection::North);
    /// 
    /// assert_eq!(is_safe_distance, true);
    /// ```
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

    /// Clear all vehicles that are out of bounds.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use context::vehicle::queue::Queue;
    /// 
    /// let mut queue = Queue::new();
    /// 
    /// queue.clear_out_of_bounds();
    /// ```
    pub fn clear_out_of_bounds(&mut self) {
        self.north.remove_out_of_bounds_vehicles();
        self.south.remove_out_of_bounds_vehicles();
        self.east.remove_out_of_bounds_vehicles();
        self.west.remove_out_of_bounds_vehicles();
    }


    //Create function that checks if vehicle in c queue is passed the turning point. If so it should shift
    pub fn shift_vehicles_at_turning_point(&mut self, ) {

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
