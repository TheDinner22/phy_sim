use std::time::Instant;

use my_first_phy::{TwoDObject, Point};
use my_first_phy::stopwatch::Watch;

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
