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
    pub fn turn_right(&mut self, vehicle: RefCell<Vehicle>) {
        let origin = vehicle.borrow().origin.clone();
        let direction = vehicle.borrow().direction.clone();
        match origin {
            Origin::East => direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.south, vehicle),
            Origin::West => direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.north, vehicle),
            Origin::North => direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.east, vehicle),
            Origin::South => direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.west, vehicle),
        }
    }
    pub fn move_vehicles(&mut self) -> Result<(), String> {
        let mut turning_queue = TurningQueue::new();
        let mut vehicle_out_of_range = TurningQueue::new();
        //North current queues
        for vehicle in &self.c_queue.north.left {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, current_vehicle.velocity);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }
        for vehicle in &self.c_queue.north.straight {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, current_vehicle.velocity);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }
        for vehicle in &self.c_queue.north.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, current_vehicle.velocity);
            if current_vehicle.point.1 >= 180 {
                turning_queue.north.right = true;
                current_vehicle.point = current_vehicle.point + Point(-10, 10);
            } else {
                self.render
                    .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
            }
        }

        if turning_queue.north.right {
            let holder_vehicle = self
                .c_queue
                .remove_first_in_queue(Origin::North, VehicleDirection::Right);
            self.turn_right(holder_vehicle);
        }

        //North After Queue

        for vehicle in &self.a_queue.north.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, current_vehicle.velocity);
            //Check if vehicle is out of the screen
            if current_vehicle.point.1 > 650 {
                vehicle_out_of_range.north.right = true;
            }
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }

        if vehicle_out_of_range.north.right {
            if self.a_queue.north.right.len() > 0 {
                self.a_queue.north.right.remove(0);
            }
        }

        //South queues
        for vehicle in &self.c_queue.south.left {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, -current_vehicle.velocity);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }
        for vehicle in &self.c_queue.south.straight {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, -current_vehicle.velocity);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }
        for vehicle in &self.c_queue.south.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, -current_vehicle.velocity);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
            if current_vehicle.point.1 <= 390 {
                turning_queue.south.right = true;
            }
        }

        if turning_queue.south.right {
            let holder_vehicle = self
                .c_queue
                .remove_first_in_queue(Origin::South, VehicleDirection::Right);

            self.turn_right(holder_vehicle);
        }

        //South After Queues
        for vehicle in &self.a_queue.south.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, -current_vehicle.velocity);
            if current_vehicle.point.1 < -40 {
                vehicle_out_of_range.south.right = true;
            }
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }

        if vehicle_out_of_range.south.right {
            if self.a_queue.south.right.len() > 0 {
                self.a_queue.south.right.remove(0);
            }
        }

        //East queues
        for vehicle in &self.c_queue.east.left {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(-current_vehicle.velocity, 0);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }
        for vehicle in &self.c_queue.east.straight {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(-current_vehicle.velocity, 0);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }
        for vehicle in &self.c_queue.east.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(-current_vehicle.velocity, 0);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
            if current_vehicle.point.0 <= 390 {
                turning_queue.east.right = true;
                current_vehicle.point = current_vehicle.point + Point(0, -10);
            }
        }

        if turning_queue.east.right {
            let holder_vehicle = self
                .c_queue
                .remove_first_in_queue(Origin::East, VehicleDirection::Right);
            self.turn_right(holder_vehicle);
        }

        //East After Queues
        for vehicle in &self.a_queue.east.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(-current_vehicle.velocity, 0);
            if current_vehicle.point.0 < -40 {
                vehicle_out_of_range.east.right = true;
            }
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }

        if vehicle_out_of_range.east.right {
            if self.a_queue.east.right.len() > 0 {
                self.a_queue.east.right.remove(0);
            }
        }

        //West current queues
        for vehicle in &self.c_queue.west.left {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(current_vehicle.velocity, 0);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }
        for vehicle in &self.c_queue.west.straight {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(current_vehicle.velocity, 0);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }
        for vehicle in &self.c_queue.west.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(current_vehicle.velocity, 0);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
            if current_vehicle.point.0 >= 180 {
                turning_queue.west.right = true;
                current_vehicle.point = current_vehicle.point + Point(10, 0);
            }
        }

        if turning_queue.west.right {
            let holder_vehicle = self
                .c_queue
                .remove_first_in_queue(Origin::West, VehicleDirection::Right);
            self.turn_right(holder_vehicle);
        }

        //West After Queues

        for vehicle in &self.a_queue.west.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(current_vehicle.velocity, 0);
            if current_vehicle.point.0 > 650 {
                vehicle_out_of_range.west.right = true;
            }
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }

        if vehicle_out_of_range.west.right {
            if self.a_queue.west.right.len() > 0 {
                self.a_queue.west.right.remove(0);
            }
        }
        Ok(())
    }
}
