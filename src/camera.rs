use rand::prelude::*;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f32,
    pub aspect: f32,
    pub aperture: f32,
    pub focus_dist: f32
}


fn random_in_unit_disk(rng: &mut rand::rngs::StdRng) -> Vec3 {
    let mut p: Vec3;
    while {
        p = &(&Vec3 {x: rng.gen(), y: rng.gen(), z: 0.0} * 2.0) - &Vec3 {x: 1.0, y: 1.0, z: 0.0};
        p.dot(&p) >= 1.0
    } {}
    p
}

impl Camera {
    fn lens_radius(&self) -> f32 {
        self.aperture / 2.0
    }
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
        &(&(&self.origin() - &(&self.u() * (self.half_width() * self.focus_dist) )) - &(&self.v() * (self.half_height() * self.focus_dist) )) - &(&self.w() * self.focus_dist)
    }
    fn horizontal(&self) -> Vec3 {
        &self.u() * (2.0 * self.half_width() * self.focus_dist)
    }
    fn vertical(&self) -> Vec3 {
        &self.v() * (2.0 * self.half_height() * self.focus_dist)
    }
    pub fn get_ray(&self, rng: &mut rand::rngs::StdRng, s: f32, t: f32) -> Ray {
        let rd    : Vec3 = &random_in_unit_disk(rng) * self.lens_radius();
        let offset: Vec3 = &(&self.u() * rd.x) + &(&self.v() * rd.y);
        Ray {
            origin: &self.origin() + &offset,
            direction: &(&(&(&self.lower_left_corner() + &(&self.horizontal() * s)) + &(&self.vertical() * t)) - &self.origin()) - &offset
        }
    }
}
