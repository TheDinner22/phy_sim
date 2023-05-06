use std::time::Instant;

use my_first_phy::{TwoDObject, Point};

struct Watch {
    instant: Instant,
    delta_t: f64
}

impl Watch {
    fn new() -> Self {
        Watch { instant: Instant::now(), delta_t: 0.0 }
    }

    fn delta_t(&mut self) -> f64 {
        let total_secs = self.instant.elapsed().as_secs_f64();
        let change_in_secs = total_secs - self.delta_t;
        self.delta_t = total_secs;
        change_in_secs
    }
}

fn main() {
    let mut clock = Watch::new();
    let o = TwoDObject::from_point_and_acceleration(Point::new(1, 1), Point::new(0, -10));
    loop {
        std::thread::sleep(std::time::Duration::new(1, 0));
        let delta_t = clock.delta_t();
        println!("{}", delta_t);
    }
}
