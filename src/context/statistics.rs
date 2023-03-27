pub struct Statistics {
    pub max_number: i32,
    pub max_velocity: i32,
    pub min_velocity: i32,
    pub longest_time: f32,
    pub shortest_time: f32,
    pub close_calls: i32,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            max_number: 0,
            max_velocity: 0,
            min_velocity: i32::MAX,
            longest_time: 0.0,
            shortest_time: f32::MAX,
            close_calls: 0,
        }
    }

    pub fn format_stats(self) -> String {
        format!("Number of cars: {}\nMax velocity: {}\nMin Velocity: {}\nShortest Time: {}\nLongest Time: {}\nClose calls: {}\nCollisions: {}", self.max_number, if self.max_velocity!= 0 { (self.max_velocity).to_string()} else {0.to_string()}, if self.min_velocity!= i32::MAX { self.min_velocity.to_string()} else {0.to_string()}, if self.shortest_time != f32::MAX { ((self.shortest_time*100.0).round()/100.0).to_string()} else {0.to_string()}, if self.longest_time != 0.0 { ((self.longest_time*100.0).round()/100.0).to_string()} else {0.to_string()}, self.close_calls, self.close_calls)
    }

    pub fn update_velocity(&mut self, v: i32) {
        if self.max_velocity < v {
            self.max_velocity = v;
        }
        if self.min_velocity > v {
            self.min_velocity = v;
        }
    }
}

pub fn stat_times(v: Vec<(f32, f32, f32, bool)>) -> (Option<f32>, Option<f32>, i32) {
    let mut h_result = -1.0;
    let mut l_result = f32::MAX;
    let mut v_removed: i32 = 0;
    for t in v {
        if t.3 {
            v_removed += 1;
        }
        if t.0 > h_result {
            h_result = t.0
        }
        if t.0 < l_result && t.0 > 0.0 {
            l_result = t.0
        }

        if t.1 > h_result {
            h_result = t.1
        }
        if t.1 < l_result && t.1 > 0.0 {
            l_result = t.1
        }

        if t.2 > h_result {
            h_result = t.2
        }
        if t.2 < l_result && t.2 > 0.0 {
            l_result = t.2
        }
    }

    let mut result = (None, None, v_removed);
    if h_result != -1.0 {
        result.0 = Some(h_result)
    }

    if l_result != f32::MAX && l_result > 0.0 {
        result.1 = Some(l_result)
    }
    result
}
