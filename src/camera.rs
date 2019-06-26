use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {

}

impl Camera {
    fn lower_left_corner(&self) -> Vec3 {
        Vec3{x: -2.0, y: -1.0, z: -1.0}
    }
    fn horizontal(&self) -> Vec3 {
        Vec3{x: 4.0, y: 0.0, z: 0.0}
    }
    fn vertical(&self) -> Vec3 {
        Vec3{x: 0.0, y: 2.0, z: 0.0}
    }
    fn origin(&self) -> Vec3 {
        Vec3{x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin(),
            direction: &(&self.lower_left_corner() + &(&self.horizontal() * u)) + &(&self.vertical() * v)
        }
    }
}
