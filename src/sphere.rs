use cgmath::Point3;
use cgmath::BaseFloat;
use std::f64;

#[derive(Copy, Clone, Debug)]
pub struct Sphere<F> {
    pub center : Point3<F>,
    pub radius : F
}

pub trait SphereHelper<F> {
    fn volume(radius: F) -> F;
}

impl SphereHelper<f64> for f64 {
    fn volume(radius: f64) -> f64 {
        return ((4.0 / 3.0) * f64::consts::PI) * radius * radius * radius;
    }
}

impl<F> Sphere<F> where F : BaseFloat + SphereHelper<F> {
    pub fn new(center: Point3<F>, radius: F) -> Sphere<F> {
        Sphere::<F> { center: center, radius: radius }
    }

    pub fn volume(self) -> F {
        return F::volume(self.radius);
    }
}
