use rand;
use rand::prelude::*;
use crate::vec3::Vec3;

pub fn random_in_unit_sphere(rng: &mut rand::rngs::StdRng) -> Vec3 {
    let mut p: Vec3;
    while {
        p = &(&Vec3{x: rng.gen(), y: rng.gen(), z: rng.gen()} * 2.0) - &Vec3{x: 1.0, y: 1.0, z: 1.0};
        p.squared_length() >= 1.0
    } {}
    p
}
