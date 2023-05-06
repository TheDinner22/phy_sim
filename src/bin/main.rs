// todos
// mass?
// collisions
// gui
// n dimensional

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
    let mut o = TwoDObject::from_point_and_acceleration(Point::new(100.0, 100.0), Point::new(-1.0, -1.0));
    loop {
        let delta_t = clock.delta_t();
        o.tick(delta_t);

        println!("({:#?}, {:#?})", o.get_pos().x, o.get_pos().y);

        std::thread::sleep(std::time::Duration::new(1, 0));
    }
}
