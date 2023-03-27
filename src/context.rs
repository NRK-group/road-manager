pub mod key;
use std::time::Instant;

pub use key::*;
pub mod render;
pub mod statistics;
pub use render::*;
pub mod vehicle;
pub use vehicle::*;

use self::statistics::Statistics;
#[allow(dead_code)]
pub struct Context {
    pub render: Render,
    pub b_queue: Queue,
    pub c_queue: Queue,
    pub a_queue: Queue,
    pub current_fastest: i32,
    pub vehicle_ids: Vec<i32>,
    pub stats: Statistics,
}

impl Context {
    pub fn new(render: Render) -> Self {
        Self {
            render,
            b_queue: Queue::new(),
            c_queue: Queue::new(),
            a_queue: Queue::new(),
            current_fastest: 0,
            vehicle_ids: Vec::new(),
            stats: Statistics::new(),
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

    pub fn turn_left(&mut self, vehicle: RefCell<Vehicle>) {
        let origin = vehicle.borrow().origin.clone();
        let vehicle_direction = vehicle.borrow().direction.clone();
        match origin {
            Origin::East => vehicle_direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.north, vehicle),
            Origin::West => vehicle_direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.south, vehicle),
            Origin::North => vehicle_direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.west, vehicle),
            Origin::South => vehicle_direction
                .clone()
                .push_to_vehicle_direction(&mut self.a_queue.east, vehicle),
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
            && self.vehicle_ids.len() <= 6
        {
            if vehicle_direction != VehicleDirection::Right {
                self.vehicle_ids.push(id);
            }
            self.c_queue.create_vehicle(origin, id, vehicle_direction)
        }
        // else {
        //     self.b_queue.create_vehicle(origin, id, vehicle_direction)
        // }
    }
    #[allow(dead_code)]
    pub fn shift_vehicle_from_bq_to_cq(&mut self) {
        let origins = [Origin::North, Origin::East, Origin::South, Origin::West];
        let vehicle_directions = [
            VehicleDirection::Left,
            VehicleDirection::Right,
            VehicleDirection::Straight,
        ];
        for origin in &origins {
            for vechicle_direction in &vehicle_directions {
                if self.vehicle_ids.len() <= 6 {
                    let v = add_vehicle_to_origin_if_safe(
                        origin,
                        vechicle_direction,
                        &mut self.c_queue,
                        &mut self.b_queue,
                    );
                    if let Some(i) = v {
                        if vechicle_direction != &VehicleDirection::Right {
                            self.vehicle_ids.push(i);
                        };
                    }
                }
            }
        }
    }
    pub fn move_vehicles(&mut self) -> Result<(), String> {
        //North current queues
        for vehicle in &self.c_queue.north.left {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, current_vehicle.velocity);
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
            if current_vehicle.point.1 >= 300 {
                current_vehicle.point = current_vehicle.point + Point(0, 10);
            }
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
            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
            if current_vehicle.point.1 >= 180 {
                current_vehicle.point = current_vehicle.point + Point(-10, 10);
            }
            self.stats.update_velocity(current_vehicle.velocity);
        }
        for q in &[
            &self.a_queue.north.right,
            &self.a_queue.north.straight,
            &self.a_queue.north.left,
        ] {
            for vehicle in q.iter() {
                let mut current_vehicle = vehicle.borrow_mut();
                current_vehicle.point = current_vehicle.point + Point(0, current_vehicle.velocity);
                self.render
                    .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
            }
        }
        //South queues
        for vehicle in &self.c_queue.south.left {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(0, -current_vehicle.velocity);

            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
            if current_vehicle.point.1 <= 270 {
                current_vehicle.point = current_vehicle.point + Point(-10, 0);
            }
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
            self.stats.update_velocity(current_vehicle.velocity);
        }
        //South After Queues
        for vehicles in &[
            &self.a_queue.south.right,
            &self.a_queue.south.straight,
            &self.a_queue.south.left,
        ] {
            for vehicle in vehicles.iter() {
                let mut current_vehicle = vehicle.borrow_mut();
                current_vehicle.point = current_vehicle.point + Point(0, -current_vehicle.velocity);
                self.render
                    .draw_vehicle(&current_vehicle, VehicleType::Verticle)?;
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
                current_vehicle.point = current_vehicle.point + Point(0, -10);
            }
            self.stats.update_velocity(current_vehicle.velocity);
        }
        //East After Queues
        for vehicles in &[
            &self.a_queue.east.right,
            &self.a_queue.east.straight,
            &self.a_queue.east.left,
        ] {
            for vehicle in vehicles.iter() {
                let mut current_vehicle = vehicle.borrow_mut();
                current_vehicle.point = current_vehicle.point + Point(-current_vehicle.velocity, 0);
                self.render
                    .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
            }
        }
        //West current queues
        for vehicle in &self.c_queue.west.left {
            let mut current_vehicle = vehicle.borrow_mut();
            current_vehicle.point = current_vehicle.point + Point(current_vehicle.velocity, 0);

            self.render
                .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
            if current_vehicle.point.0 >= 300 {
                current_vehicle.point = current_vehicle.point + Point(10, -10);
            }
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
                current_vehicle.point = current_vehicle.point + Point(10, 0);
            }
            self.stats.update_velocity(current_vehicle.velocity);
        }
        //West After Queues
        for vehicles in &[
            &self.a_queue.west.right,
            &self.a_queue.west.straight,
            &self.a_queue.west.left,
        ] {
            for vehicle in vehicles.iter() {
                let mut current_vehicle = vehicle.borrow_mut();
                current_vehicle.point = current_vehicle.point + Point(current_vehicle.velocity, 0);
                self.render
                    .draw_vehicle(&current_vehicle, VehicleType::Horizontal)?;
            }
        }
        self.shift_vehicles_at_turning_point();
        Ok(())
    }

    pub fn remove_vehicles(&mut self) {
        //Update the highest and lowest ties
        let times = self.a_queue.clear_out_of_bounds();
        if let Some(highest) = times.0 {
            if highest > self.stats.longest_time {
                self.stats.longest_time = highest
            }
        }
        if let Some(lowest) = times.1 {
            if lowest < self.stats.shortest_time {
                self.stats.shortest_time = lowest
            }
        }

        self.stats.max_number += times.2;
    }

    //Create function that checks if vehicle in c queue is passed the turning point. If so it should shift
    pub fn shift_vehicles_at_turning_point(&mut self) {
        //shift North
        if let Some(v) = self.c_queue.north.right.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.north.right.remove(0);
                self.turn_right(vehicle_to_shift);
            }
        }
        if let Some(v) = self.c_queue.north.straight.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.north.straight.remove(0);
                self.turn_straight(vehicle_to_shift);
                self.vehicle_ids.remove(0);
            }
        }
        if let Some(v) = self.c_queue.north.left.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.north.left.remove(0);
                self.turn_left(vehicle_to_shift);
                self.vehicle_ids.remove(0);
            }
        }

        //Shift South
        if let Some(v) = self.c_queue.south.right.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.south.right.remove(0);
                self.turn_right(vehicle_to_shift);
            }
        }
        if let Some(v) = self.c_queue.south.straight.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.south.straight.remove(0);
                self.turn_straight(vehicle_to_shift);

                self.vehicle_ids.remove(0);
            }
        }
        if let Some(v) = self.c_queue.south.left.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.south.left.remove(0);
                self.turn_left(vehicle_to_shift);
                self.vehicle_ids.remove(0);
            }
        }

        //shift East
        if let Some(v) = self.c_queue.east.right.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.east.right.remove(0);
                self.turn_right(vehicle_to_shift);
            }
        }
        if let Some(v) = self.c_queue.east.straight.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.east.straight.remove(0);
                self.turn_straight(vehicle_to_shift);
                self.vehicle_ids.remove(0);
            }
        }
        if let Some(v) = self.c_queue.east.left.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.east.left.remove(0);
                self.turn_left(vehicle_to_shift);
                self.vehicle_ids.remove(0);
            }
        }

        //Shift West
        if let Some(v) = self.c_queue.west.right.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.west.right.remove(0);
                self.turn_right(vehicle_to_shift);
            }
        }
        if let Some(v) = self.c_queue.west.straight.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.west.straight.remove(0);
                self.turn_straight(vehicle_to_shift);
                self.vehicle_ids.remove(0);
            }
        }
        if let Some(v) = self.c_queue.west.left.first() {
            if v.borrow().turn() {
                //Shift from c queue to a queue
                let vehicle_to_shift = self.c_queue.west.left.remove(0);
                self.turn_left(vehicle_to_shift);
                self.vehicle_ids.remove(0);
            }
        }
    }

    pub fn speed_up_fastest(&mut self) {
        if let Some(id) = self.vehicle_ids.first() {
            if self.current_fastest != *id {
                //Speed up this car
                //Check north S
                if let Some(v) = self.c_queue.north.straight.first() {
                    if v.borrow().id == *id {
                        v.borrow_mut().velocity = 10;

                        self.current_fastest = *id;
                    }
                    self.stats.update_velocity(v.borrow().velocity)
                };
                //Check north L
                if let Some(v) = self.c_queue.north.left.first() {
                    if v.borrow().id == *id {
                        v.borrow_mut().velocity = 10;

                        self.current_fastest = *id;
                    }
                    self.stats.update_velocity(v.borrow().velocity)
                };
                //Check east S
                if let Some(v) = self.c_queue.east.straight.first() {
                    if v.borrow().id == *id {
                        v.borrow_mut().velocity = 10;

                        self.current_fastest = *id;
                    }
                    self.stats.update_velocity(v.borrow().velocity)
                };
                //Check; east L
                if let Some(v) = self.c_queue.east.left.first() {
                    if v.borrow().id == *id {
                        v.borrow_mut().velocity = 10;

                        self.current_fastest = *id;
                    }
                    self.stats.update_velocity(v.borrow().velocity)
                };
                //Check south S
                if let Some(v) = self.c_queue.south.straight.first() {
                    if v.borrow().id == *id {
                        v.borrow_mut().velocity = 10;

                        self.current_fastest = *id;
                    }
                    self.stats.update_velocity(v.borrow().velocity)
                };
                //Check south L
                if let Some(v) = self.c_queue.south.left.first() {
                    if v.borrow().id == *id {
                        v.borrow_mut().velocity = 10;

                        self.current_fastest = *id;
                    }
                    self.stats.update_velocity(v.borrow().velocity)
                };
                //Check west S
                if let Some(v) = self.c_queue.west.straight.first() {
                    if v.borrow().id == *id {
                        v.borrow_mut().velocity = 10;

                        self.current_fastest = *id;
                    }
                    self.stats.update_velocity(v.borrow().velocity)
                };
                //Check west L
                if let Some(v) = self.c_queue.west.left.first() {
                    if v.borrow().id == *id {
                        v.borrow_mut().velocity = 10;

                        self.current_fastest = *id;
                    };
                    self.stats.update_velocity(v.borrow().velocity)
                };
            }
        };
    }
}

fn add_vehicle_to_origin_if_safe(
    origin: &Origin,
    vehicle_direction: &VehicleDirection,
    c_queue: &mut Queue,
    b_queue: &mut Queue,
) -> Option<i32> {
    if c_queue.is_safe_distance_from_last_vehicle(origin, vehicle_direction)
        && origin.get_len_of_queue_from_direction(b_queue, vehicle_direction) != 0
    {
        let vehicle = b_queue.remove_first_in_queue(origin, vehicle_direction);
        vehicle.borrow_mut().start = Instant::now();
        let id = vehicle.borrow().id;
        origin.add_vehicle_to_origin(vehicle_direction, c_queue, vehicle);
        return Some(id);
    }
    None
}
