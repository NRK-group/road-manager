pub mod key;
pub use key::*;
pub mod render;
pub use render::*;
pub mod vehicle;
pub use vehicle::*;
pub struct Context {
    pub render: Render,
    pub b_queue: Queue,
    pub c_queue: Queue,
    pub a_queue: Queue,
}

impl Context {
    pub fn new(render: Render) -> Self {
        Self {
            render,
            b_queue: Queue::new(),
            c_queue: Queue::new(),
            a_queue: Queue::new(),
        }
    }
    pub fn turn_right(&mut self, vehicle: RefCell<Vehicle>) {
        let origin = vehicle.borrow().origin.clone();
        let vehicle_direction = vehicle.borrow().direction.clone();
        match origin {
            Origin::East => vehicle_direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.south, vehicle),
            Origin::West => vehicle_direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.north, vehicle),
            Origin::North => vehicle_direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.east, vehicle),
            Origin::South => vehicle_direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.west, vehicle),
        }
    }
    pub fn turn_straight(&mut self, vehicle: RefCell<Vehicle>) {
        let origin = vehicle.borrow().origin.clone();
        let vehicle_direction = vehicle.borrow().direction.clone();
        origin.add_vehicle_to_origin(&vehicle_direction, &mut self.a_queue, vehicle)
    }
    pub fn add_vehicle_to_queue(&mut self, origin: Origin, id: i32) {
        let vehicle_direction = VehicleDirection::random();
        if self
            .c_queue
            .is_safe_distance_from_last_vehicle(&origin, &vehicle_direction)
            && origin.get_len_of_queue_from_direction(&self.b_queue, &vehicle_direction) == 0
        {
            self.c_queue.create_vehicle(origin, id, vehicle_direction)
        } else {
            self.b_queue.create_vehicle(origin, id, vehicle_direction)
        }
    }
    pub fn shift_vehicle_from_bq_to_cq(&mut self) {
        let origins = [Origin::North, Origin::East, Origin::South, Origin::West];
        let vehicle_directions = [
            VehicleDirection::Left,
            VehicleDirection::Right,
            VehicleDirection::Straight,
        ];
        for origin in &origins {
            for vechicle_direction in &vehicle_directions {
                add_vehicle_to_origin_if_safe(
                    origin,
                    vechicle_direction,
                    &mut self.c_queue,
                    &mut self.b_queue,
                );
            }
        }
    }
    pub fn move_vehicles(&mut self) -> Result<(), String> {
        let mut turning_queue = TurningQueue::new();
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
            if current_vehicle.point.1 >= 380 {
                turning_queue.north.straight = true
            }
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }
        if turning_queue.north.straight {
            let holder_vehicle = self
                .c_queue
                .remove_first_in_queue(&Origin::North, &VehicleDirection::Straight);
            self.turn_straight(holder_vehicle);
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
                .remove_first_in_queue(&Origin::North, &VehicleDirection::Right);
            self.turn_right(holder_vehicle);
        }

        //North After Queue
        for vehicle in &self.a_queue.north.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, current_vehicle.velocity);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
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
            if current_vehicle.point.1 <= 220 {
                turning_queue.south.straight = true;
            }
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
        }
        if turning_queue.south.straight {
            let holder_vehicle = self
                .c_queue
                .remove_first_in_queue(&Origin::South, &VehicleDirection::Straight);
            self.turn_straight(holder_vehicle);
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
                .remove_first_in_queue(&Origin::South, &VehicleDirection::Right);

            self.turn_right(holder_vehicle);
        }

        //South After Queues
        for vehicle in &self.a_queue.south.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, -current_vehicle.velocity);
           
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
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
            if current_vehicle.point.0 <= 220 {
                turning_queue.east.straight = true;
            }
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }
        if turning_queue.east.straight {
            let holder_vehicle = self
                .c_queue
                .remove_first_in_queue(&Origin::East, &VehicleDirection::Straight);
            self.turn_straight(holder_vehicle);
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
                .remove_first_in_queue(&Origin::East, &VehicleDirection::Right);
            self.turn_right(holder_vehicle);
        }

        //East After Queues
        for vehicle in &self.a_queue.east.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(-current_vehicle.velocity, 0);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
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
            if current_vehicle.point.0 >= 340 {
                turning_queue.west.straight = true
            }
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }
        if turning_queue.west.straight {
            let holder_vehicle = self
                .c_queue
                .remove_first_in_queue(&Origin::West, &VehicleDirection::Straight);
            self.turn_straight(holder_vehicle);
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
                .remove_first_in_queue(&Origin::West, &VehicleDirection::Right);
            self.turn_right(holder_vehicle);
        }

        //West After Queues

        for vehicle in &self.a_queue.west.right {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(current_vehicle.velocity, 0);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
        }
        self.a_queue.clear_out_of_bounds();

        Ok(())
    }
}
fn add_vehicle_to_origin_if_safe(
    origin: &Origin,
    vehicle_direction: &VehicleDirection,
    c_queue: &mut Queue,
    b_queue: &mut Queue,
) {
    if c_queue.is_safe_distance_from_last_vehicle(origin, vehicle_direction)
        && origin.get_len_of_queue_from_direction(b_queue, vehicle_direction) != 0
    {
        let vehicle = b_queue.remove_first_in_queue(origin, vehicle_direction);
        origin.add_vehicle_to_origin(vehicle_direction, c_queue, vehicle);
    }
}
