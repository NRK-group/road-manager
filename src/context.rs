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
}
