use cgmath::Point3;
use cgmath::Vector3;
use cgmath::Quaternion;
use cgmath::BaseFloat;

#[derive(Copy, Clone, Debug)]
pub struct Camera<F> {
    pub position : Point3<F>,
    pub orientation : Quaternion<F>,
    pub field_of_view : F
}

impl<F> Camera<F> where F : BaseFloat {
    pub fn new(position: Point3<F>, orientation: Quaternion<F>, fov: F) -> Camera<F> {
        Camera::<F> { position: position, orientation: orientation, field_of_view: fov }
    }

    pub fn up(self) -> Vector3<F> {
        return self.orientation * Vector3::<F>::unit_y();
    }
}
