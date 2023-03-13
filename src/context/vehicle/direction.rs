use rand::Rng;
pub use sdl2::pixels::Color;
#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
    Straight,
}

impl Direction {
    pub fn random() -> (Self, Color) {
        match rand::thread_rng().gen_range(1..=3) {
            1 => (Self::Left, Color::BLUE),
            2 => (Self::Right, Color::RED),
            _ => (Self::Straight, Color::GREEN),
        }
    }
}
