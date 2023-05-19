use std::time::Instant;

pub struct Watch {
    instant: Instant,
    delta_t: f64
}

impl Watch {
    pub fn new() -> Self {
        Watch { instant: Instant::now(), delta_t: 0.0 }
    }

    pub fn delta_t(&mut self) -> f64 {
        let total_secs = self.instant.elapsed().as_secs_f64();
        let change_in_secs = total_secs - self.delta_t;
        self.delta_t = total_secs;
        change_in_secs
    }
}
