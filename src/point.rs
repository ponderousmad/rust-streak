use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

impl Point {
    // Once this is supported: #![feature(associated_consts)]
    // const w : f64 = 1;
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}
