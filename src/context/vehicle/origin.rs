
use rand::Rng;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Origin {
    North,
    East,
    South,
    West,
}

impl Origin {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(1..=4) {
            1 => Self::North,
            2 => Self::South,
            3 => Self::West,
            _ => Self::East,
        }
    }
}
