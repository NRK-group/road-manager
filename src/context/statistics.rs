pub struct Statistics {
    max_number: i32,
    max_velocity: i32,
    min_velocity: i32,
    longest_time: i32,
    shortest_time: i32,
    close_calls: i32,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            max_number: 0,
            max_velocity: 0,
            min_velocity: 0,
            longest_time: 0,
            shortest_time: i32::MAX,
            close_calls: 0,
        }
    }
}
