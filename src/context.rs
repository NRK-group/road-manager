pub mod key;
pub use key::*;
pub mod render;
pub use render::*;
pub mod vehicle;
pub use vehicle::*;

pub struct Context {
    pub render: Render,
    pub c_queue: Queue,
    pub a_queue: Queue,
}



impl Context {
    pub fn new(render: Render) -> Self {
        Self {
            render,
            c_queue: Queue::new(),
            a_queue: Queue::new(),
        }
    }

    pub fn move_vehicles(&mut self) -> Result<(), String> {
      
let mut turning_queue = TurningQueue::new();
        //North queues
        for vehicle in &self.c_queue.north.left {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, 4);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }
        for vehicle in &self.c_queue.north.straight {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, 4);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }
        for vehicle in &self.c_queue.north.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, 4);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
            if current_vehicle.point.1 >= 180 {
                turning_queue.north.right = true;
            }
        }

        if  turning_queue.north.right {
            let holder_vehicle = self.remove_from_c_queue(Origin::North, VehicleDirection::Right);
            self.turn_right(holder_vehicle);
        }

        //South queues
        for vehicle in &self.c_queue.south.left {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, -1);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }
        for vehicle in &self.c_queue.south.straight {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, -1);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }
        for vehicle in &self.c_queue.south.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, -1);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
            if current_vehicle.point.1 <= 380 {
                turning_queue.south.right = true;
            }
        }

        if  turning_queue.south.right {
            let holder_vehicle = self.remove_from_c_queue(Origin::South, VehicleDirection::Right);
            self.turn_right(holder_vehicle);
        }

        //East queues
        for vehicle in &self.c_queue.east.left {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(-1, 0);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }
        for vehicle in &self.c_queue.east.straight {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(-1, 0);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }
        for vehicle in &self.c_queue.east.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(-1, 0);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
            if current_vehicle.point.0 <= 380 {
                turning_queue.east.right = true;
            }
        }

        if  turning_queue.east.right {
            let holder_vehicle = self.remove_from_c_queue(Origin::East, VehicleDirection::Right);
            self.turn_right(holder_vehicle);
        }

        //West queues
        for vehicle in &self.c_queue.west.left {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(1, 0);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }
        for vehicle in &self.c_queue.west.straight {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(1, 0);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }
        for vehicle in &self.c_queue.west.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(1, 0);
            self.render.draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
            if current_vehicle.point.0 >= 180 {
                turning_queue.west.right = true;
            }
        }

        if  turning_queue.west.right {
            let holder_vehicle = self.remove_from_c_queue(Origin::West, VehicleDirection::Right);
            self.turn_right(holder_vehicle);
        }
        Ok(())
    }

    pub fn turn_right(&mut self, vehicle: RefCell<Vehicle>) {
        let origin = vehicle.borrow().origin.clone();
        match origin {
            Origin::East => self.a_queue.south.add_vehicle_to_queue_with_refcell(vehicle),
            Origin::West => self.a_queue.north.add_vehicle_to_queue_with_refcell(vehicle),
            Origin::North => self.a_queue.east.add_vehicle_to_queue_with_refcell(vehicle),
            Origin::South => self.a_queue.west.add_vehicle_to_queue_with_refcell(vehicle),
        }
    }

    pub fn remove_from_c_queue(
        &mut self,
        origin: Origin,
        direction: VehicleDirection
    ) -> RefCell<Vehicle> {
        match origin {
            Origin::East => {
                match direction {
                    VehicleDirection::Left => self.c_queue.east.left.remove(0),
                    VehicleDirection::Straight => self.c_queue.east.straight.remove(0),
                    VehicleDirection::Right => self.c_queue.east.right.remove(0),
                }
            }
            Origin::West => {
                match direction {
                    VehicleDirection::Left => self.c_queue.west.left.remove(0),
                    VehicleDirection::Straight => self.c_queue.west.straight.remove(0),
                    VehicleDirection::Right => self.c_queue.west.right.remove(0),
                }
            }

            Origin::North => {
                match direction {
                    VehicleDirection::Left => self.c_queue.north.left.remove(0),
                    VehicleDirection::Straight => self.c_queue.north.straight.remove(0),
                    VehicleDirection::Right => self.c_queue.north.right.remove(0),
                }
            }

            Origin::South => {
                match direction {
                    VehicleDirection::Left => self.c_queue.south.left.remove(0),
                    VehicleDirection::Straight => self.c_queue.south.straight.remove(0),
                    VehicleDirection::Right => self.c_queue.south.right.remove(0),
                }
            }
        }
    }
}