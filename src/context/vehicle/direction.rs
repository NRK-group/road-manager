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
    /// Returns a new direction.
    ///
    /// # Returns
    ///
    /// A new direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::direction::Direction;
    ///
    /// let direction = Direction::new();
    /// ```
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
            straight: Vec::new(),
        }
    }
    /// Returns the first vehicle in the specified direction.
    ///
    /// # Arguments
    ///
    /// * `vehicle_direction` - The direction of the vehicles in the queue.
    ///
    /// # Returns
    ///
    /// The vehicle that get removes in the specified direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::direction::Direction;
    /// use context::vehicle::direction::VehicleDirection;
    ///
    /// let mut direction = Direction::new();
    /// let vehicle = direction.remove_first_from_direction(&VehicleDirection::Left);
    /// ```
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
    /// Remove vehicles that are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::direction::Direction;
    ///
    /// let mut direction = Direction::new();
    /// direction.remove_out_of_bounds_vehicles();
    /// ```
    pub fn remove_out_of_bounds_vehicles(&mut self) -> (f32, f32, f32, bool) {
        //Check each lane
        let mut r: f32 = -1.0;
        let mut s: f32 = -1.0;
        let mut l: f32 = -1.0;
        let mut result = false;
        if let Some(vehicle) = self.right.first() {
            let points = vehicle.borrow().point;
            if points.1 > 650 || points.1 < -40 || points.0 < -40 || points.0 > 650 {
                r = self.right.remove(0).borrow().start.elapsed().as_secs_f32();
                result = true;
            }
        };
        if let Some(vehicle) = self.straight.first() {
            let points = vehicle.borrow().point;
            if points.1 > 650 || points.1 < -40 || points.0 < -40 || points.0 > 650 {
                s = self
                    .straight
                    .remove(0)
                    .borrow()
                    .start
                    .elapsed()
                    .as_secs_f32();
                result = true;
            }
        };
        if let Some(vehicle) = self.left.first() {
            let points = vehicle.borrow().point;
            if points.1 > 650 || points.1 < -40 || points.0 < -40 || points.0 > 650 {
                l = self.left.remove(0).borrow().start.elapsed().as_secs_f32();
                result = true;
            }
        };

        (l, s, r, result)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VehicleDirection {
    Left,
    Straight,
    Right,
}

impl VehicleDirection {
    /// Returns a random vehicle direction.
    ///
    /// # Returns
    ///
    /// A random vehicle direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::direction::VehicleDirection;
    ///
    /// let vehicle_direction = VehicleDirection::random();
    /// ```
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(1..=3) {
            1 => Self::Left,
            2 => Self::Right,
            _ => Self::Straight,
        }
    }
    /// Pushes a vehicle to the specified vehicle direction.
    ///
    /// # Arguments
    ///     
    /// * `direction` - The direction of the vehicles in the queue.
    /// * `vehicle` - The vehicle that will be pushed to the specified direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::direction::VehicleDirection;
    /// use context::vehicle::direction::Direction;
    /// use context::vehicle::Vehicle;
    /// use std::cell::RefCell;
    ///
    /// let mut direction = Direction::new();
    /// let vehicle = Vehilce::new(Origin::new(0, 0), &VehicleDirection::Left, 0);
    ///
    /// direction.push_to_vehicle_direction(&VehicleDirection::Left, RefCell::new(vehicle));
    /// ```
    pub fn push_to_vehicle_direction(&self, direction: &mut Direction, vehicle: RefCell<Vehicle>) {
        match self {
            VehicleDirection::Left => direction.left.push(vehicle),
            VehicleDirection::Straight => direction.straight.push(vehicle),
            VehicleDirection::Right => direction.right.push(vehicle),
        }
    }

    /// Returns the length of the specified vehicle direction.
    ///
    /// # Arguments
    ///
    /// * `direction` - The direction of the vehicles in the queue.
    ///
    /// # Returns
    ///
    /// The length of the specified vehicle direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use context::vehicle::direction::VehicleDirection;
    /// use context::vehicle::direction::Direction;
    ///
    /// let vehicle_direction = VehicleDirection::Left;
    /// let direction = Direction::new();
    ///
    /// let len = vehicle_direction.get_len_of_vehicle_direction(&direction);
    ///
    pub fn get_len_of_vehicle_direction(&self, direction: &Direction) -> usize {
        match self {
            VehicleDirection::Left => direction.left.len(),
            VehicleDirection::Straight => direction.straight.len(),
            VehicleDirection::Right => direction.right.len(),
        }
    }
}
