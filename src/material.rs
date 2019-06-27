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
    pub albedo: Color3,
    pub f: f32
}

impl MetalMaterial {
    fn fuzz(&self) -> f32 {
        if self.f < 1.0 {self.f} else {1.0}
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(&(n * 2.0) * v.dot(n))
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv: Vec3 = v.unit_vector();
    let dt: f32 = uv.dot(n);
    let discriminant: f32 = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(
            &(&(&uv - &(n * dt)) * ni_over_nt) - &(n * discriminant.sqrt())
        )
    } else {
        None
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, rng: &mut rand::rngs::StdRng, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected: Vec3 = reflect(&r_in.direction.unit_vector(), &hit_record.normal);
        let scattered: Ray  = Ray {
            origin: hit_record.p,
            direction: &reflected + &(&util::random_in_unit_sphere(rng) * self.fuzz())
        };
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

pub struct DielectricMaterial {
    pub ref_idx: f32
}

impl Material for DielectricMaterial {
    fn scatter(&self, _rng: &mut rand::rngs::StdRng, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let attenuation: Color3 = Color3 {r: 1.0, g: 1.0, b: 1.0};
        let (outward_normal, ni_over_nt) =
            if r_in.direction.dot(&hit_record.normal) > 0.0 {
                (-&hit_record.normal, self.ref_idx)
            } else {
                (hit_record.normal, 1.0 / self.ref_idx)
            };

        refract(&r_in.direction, &outward_normal, ni_over_nt).map(|refracted| {
           ScatterRecord {
               attenuation,
               scattered: Ray{origin: hit_record.p, direction: refracted}
           }
        })
    }
}
