use cgmath::Point3;
use cgmath::Vector3;
use cgmath::Quaternion;
use cgmath::BaseFloat;
use num::traits::Float;

#[derive(Copy, Clone, Debug)]
pub struct Camera<F> {
    pub position : Point3<F>,
    pub orientation : Quaternion<F>,
    pub field_of_view : F
}

impl<F> Camera<F> where F : BaseFloat + Float {
    pub fn new(position: Point3<F>, orientation: Quaternion<F>, fov: F) -> Camera<F> {
        Camera::<F> { position: position, orientation: orientation, field_of_view: fov }
    }

    pub fn up(self) -> Vector3<F> {
        return self.orientation * Vector3::<F>::unit_y();
    }

    /// Calculate the ray for a given 'pixel'
    /// x and y are in normalized device offsets in the range [-1, 1]
    /// or less for a non-square aspect ratio.
    pub fn ray(self, x: F, y: F) -> Vector3<F> {
        let z : F = F::one() / F::tan(self.field_of_view / (F::one() + F::one()));
        let ray = Vector3::<F>::new(x, y, z);
        return self.orientation * ray;
    }
}
