use std::ops::{Add, Sub, Mul};

mod objects;
pub mod stopwatch;

trait Distance {
    // distance between here and somewhere else
    // NOT optimized in the slightest and is terrible
    // if self or other is empty, returns 0
    fn distance(&self, other: impl Distance) -> f64 {
        let p1s = self.points();
        let p2s = other.points();

        // compare each point in p1s to each point in p2s
        let mut ps: Vec<(&Point, &Point)> = Vec::with_capacity(p1s.len() * p2s.len());

        for p1 in p1s {
            for p2 in &p2s {
                ps.push((p1, *p2));
            }
        }

        ps.into_iter()
            .map(|(p1, p2)| {
                // cringe ass rust cant square an f64
                let dist_squared = (p1.x - p2.x)*(p1.x - p2.x) + (p1.y - p2.y)*(p1.y - p2.y);
                (dist_squared as f64).sqrt()
            })
            // cannot just use min :(
            .reduce(|acc, dist| if dist < acc { dist } else { acc })
            .unwrap_or(0f64)
    }

    // need to be able to quantify lines, shapes, solids, etc. as
    // a set of points to calculate distance
    fn points(&self) -> Vec<&Point>;
}

#[derive(Default, Clone, Copy, Debug)]
pub struct TwoDObject {
    position: Point,
    velocity: Vector,
    acceleration: Vector,
}

impl TwoDObject {
    // temp function that makes an object at pos with a velocity of 0'
    // and an acceleration vector from (0,0) to temp
    pub fn from_point_and_acceleration(pos: Point, temp: Point) -> Self {
        TwoDObject { position: pos, velocity: Vector::default(), acceleration: Vector { terminal: Point::default(), tip: temp } }
    }

    // TODO should this change pos or vel first?????
    pub fn tick(&mut self, secs_passed: f64) {
        // update the pos via vel
        self.position = self.position + self.velocity * secs_passed;
        
        // update the vel via acc
        self.velocity = self.velocity + self.acceleration * secs_passed;
    }

    pub fn get_pos(&self) -> &Point {
        &self.position
    }
}

// TODO what about an N-dim point?
#[derive(Default, Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl Distance for Point {
    fn points(&self) -> Vec<&Point> {
        vec![&self]
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        let (vec_x, vec_y) = rhs.serialize();
        let vec_as_point = Point::new(vec_x, vec_y);
        self + vec_as_point
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}

// TODO what about an N-dim vector?
#[derive(Default, Clone, Copy, Debug)]
pub struct Vector {
    terminal: Point,
    tip: Point,
}

impl Vector {
    fn serialize(&self) -> (f64, f64) {
        (self.tip.x - self.terminal.x, self.tip.y - self.terminal.y)
    }

    fn magnitude(&self) -> f64 {
        let (a,b) = self.serialize();
        let magnitude_squared = a + b;
        (magnitude_squared as f64).sqrt()
    }

    // TODO is this correct?
    fn dot_produtct(&self, other: &Vector) -> f64 {
        let (x1, y1) = self.serialize();
        let (x2, y2) = other.serialize();

        x1*x2 + y1*y2
    }

    fn cross_produtct(&self, other: &Vector) -> Vector { todo!() }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector { terminal: self.terminal + rhs.terminal, tip: self.tip + rhs.tip }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector { terminal: self.terminal - rhs.terminal, tip: self.tip - rhs.tip }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector { terminal: self.terminal * rhs, tip: self.tip * rhs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
