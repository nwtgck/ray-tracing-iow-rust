use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
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
    fn origin(&self) -> Vec3 {
        self.lookfrom
    }
    fn w(&self) -> Vec3 {
        (&self.lookfrom - &self.lookat).unit_vector()
    }
    fn u(&self) -> Vec3 {
        self.vup.cross(&self.w()).unit_vector()
    }
    fn v(&self) -> Vec3 {
        self.w().cross(&self.u())
    }

    fn lower_left_corner(&self) -> Vec3 {
        &(&(&self.origin() - &(&self.u() * self.half_height())) - &(&self.v() * self.half_height())) - &self.w()
    }
    fn horizontal(&self) -> Vec3 {
        &self.u() * (2.0 * self.half_width())
    }
    fn vertical(&self) -> Vec3 {
        &self.v() * (2.0 * self.half_height())
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray {
            origin: self.origin(),
            direction: &(&(&self.lower_left_corner() + &(&self.horizontal() * s)) + &(&self.vertical() * t)) - &self.origin()
        }
    }
}
