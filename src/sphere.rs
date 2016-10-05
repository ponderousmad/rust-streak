use cgmath::Point3;
use std::f64;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center : Point3<f64>,
    pub radius : f64
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Sphere {
        Sphere { center: center, radius: radius }
    }

    pub fn volume(self) -> f64 {
        return (4.0 * f64::consts::PI / 3.0) * self.radius * self.radius * self.radius;
    }
}
