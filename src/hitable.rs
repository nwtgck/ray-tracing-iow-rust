use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
