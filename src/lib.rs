pub mod point;
pub mod sphere;
pub mod camera;

extern crate cgmath;

#[cfg(test)]
mod tests {
    use point::Point;
    use sphere::Sphere;
    use camera::Camera;

    use std::f64;

    use cgmath::Matrix4;
    use cgmath::Vector4;
    use cgmath::Vector3;
    use cgmath::Point3;
    use cgmath::Quaternion;
    use cgmath::Transform;

    type Mat4 = Matrix4<f64>;
    type Vec4 = Vector4<f64>;

    #[test]
    fn apply_matrix() {
        let p = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let i = Mat4::one();
        assert!(i * p == p);
    }

    #[test]
    fn it_works() {
        let origin = Point { x: 0.0, y: 0.0, z: 0.0 };
        let p = Point { x: 1.0, y: 1.0, z: 1.0 };
        let still_p = origin + p;
        assert!(still_p.x == p.x);
    }

    #[test]
    fn sphere_volume() {
        let center = Point3::<f64>::new(1.0, 1.0, 1.0);
        let s = Sphere::<f64>::new(center, 10.0);
        assert!(s.volume() == (4000.0 * f64::consts::PI / 3.0));
    }

    #[test]
    fn camera() {
        let position = Point3::<f64>::new(1.0, 1.0, 1.0);
        let c = Camera::<f64>::new(position, Quaternion::<f64>::new(1.0, 0.0, 0.0, 0.0), 50.0);
        let up = c.up();
        assert!(up == Vector3::<f64>::unit_y());
    }
}
