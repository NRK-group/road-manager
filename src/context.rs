pub mod key;
pub use key::*;
pub mod render;
pub use render::*;
pub mod vehicle;
pub use vehicle::*;

pub struct Context {
    pub render: Render,
    pub velocity: u32,
    pub gap: u32, //If car in front is not going straight gap * 2
    pub b_queue: (usize, usize, usize, usize),
    pub amber: Option<Origin>,
    pub light: Option<Origin>,
}

impl Context {
    pub fn new(render: Render) -> Self {
        Self {
            render,
            velocity: 50,
            gap: 50,
            b_queue: (0, 0, 0, 0),
            amber: None,
            light: None,
        }
    }
}
