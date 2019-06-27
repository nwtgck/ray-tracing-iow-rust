use crate::color3::Color3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vec3::Vec3;
use crate::util;

pub struct ScatterRecord{
    pub attenuation: Color3,
    pub scattered: Ray
}

pub trait Material {
    fn scatter(&self, rng: &mut rand::rngs::StdRng, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

pub struct LambertMaterial {
    pub albedo: Color3
}

impl Material for LambertMaterial {
    fn scatter(&self, rng: &mut rand::rngs::StdRng, _r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let target: Vec3 = &(&hit_record.p + &hit_record.normal) + &util::random_in_unit_sphere(rng);
        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray{origin: hit_record.p, direction: &target - &hit_record.p}
        })
    }
}

pub struct MetalMaterial {
    pub albedo: Color3
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(&(n * 2.0) * v.dot(n))
}

impl Material for MetalMaterial {
    fn scatter(&self, _rng: &mut rand::rngs::StdRng, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected: Vec3 = reflect(&r_in.direction.unit_vector(), &hit_record.normal);
        let scattered: Ray  = Ray {origin: hit_record.p, direction: reflected};
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered: scattered
            })
        } else {
            None
        }
    }
}
