pub mod key;
pub use key::*;
pub mod render;
pub use render::*;
pub mod vehicle;
pub use vehicle::*;

pub struct Context {
    pub render: Render,
    pub c_queue: Queue,
}

impl Context {
    pub fn new(render: Render) -> Self {
        Self {
            render,
            c_queue: Queue::new(),
        }
    }

    pub fn move_vehicles(&mut self) -> Result<(), String> {
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
        }

        Ok(())
    }
}
