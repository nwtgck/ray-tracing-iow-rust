use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub vfov: f32,
    pub aspect: f32
}

impl Camera {
    fn theta(&self) -> f32 {
        self.vfov * std::f32::consts::PI / 180.0
    }
    fn half_height(&self) -> f32 {
        (self.theta() / 2.0).tan()
    }
    fn half_width(&self) -> f32 {
        self.aspect * self.half_height()
    }
    fn lower_left_corner(&self) -> Vec3 {
        Vec3{x: -self.half_width(), y: -self.half_height(), z: -1.0}
    }
    fn horizontal(&self) -> Vec3 {
        Vec3{x: 2.0 * self.half_width(), y: 0.0, z: 0.0}
    }
    fn vertical(&self) -> Vec3 {
        Vec3{x: 0.0, y: 2.0 * self.half_height(), z: 0.0}
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
