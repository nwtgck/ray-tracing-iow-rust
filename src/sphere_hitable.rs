use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::Hitable;
use crate::hitable::HitRecord;

pub struct SphereHitable {
    pub center: Vec3,
    pub radius: f32
}

impl Hitable for SphereHitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = &r.origin - &self.center;
        let a : f32  = r.direction.dot(&r.direction);
        let b : f32  = oc.dot(&r.direction);
        let c : f32  = oc.dot(&oc) - self.radius*self.radius;
        let discriminant: f32 = b * b - a * c;

        let temp1: f32  = (-b - discriminant.sqrt()) / a;
        let temp2: f32  = (-b + discriminant.sqrt()) / a;
        let b1   : bool = t_min < temp1 && temp1 < t_max;
        let b2   : bool = t_min < temp2 && temp2 < t_max;
        if discriminant > 0.0 && (b1 || b2) {
            let t: f32  = if b1 { temp1 } else {temp2};
            let p: Vec3 = r.point_at_parameter(t);
            let normal: Vec3 = &(&p - &self.center) / self.radius;
            Some(HitRecord{ t, p, normal })
        } else {
            None
        }
    }
}
