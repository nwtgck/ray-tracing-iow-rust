use std::io;
use std::io::Write;
use rand;
use rand::prelude::*;

mod color3;
mod vec3;
mod ray;
mod hitable;
mod list_hitable;
mod sphere_hitable;
mod camera;

use color3::Color3;
use vec3::Vec3;
use ray::Ray;
use hitable::Hitable;
use list_hitable::ListHitable;
use sphere_hitable::SphereHitable;

fn random_in_unit_sphere(rng: &mut rand::rngs::StdRng) -> Vec3 {
    let mut p: Vec3;
    while {
        p = &(&Vec3{x: rng.gen(), y: rng.gen(), z: rng.gen()} * 2.0) - &Vec3{x: 1.0, y: 1.0, z: 1.0};
        p.squared_length() >= 1.0
    } {}
    p
}

use camera::Camera;

fn color<H: Hitable>(rng: &mut rand::rngs::StdRng, r: &Ray, hitable: &H) -> Color3 {
    if let Some(hit_record) = hitable.hit(r, 0.0, std::f32::MAX) {
        let target: Vec3 = &(&hit_record.p + &hit_record.normal) + &random_in_unit_sphere(rng);
        &color(rng, &Ray{
            origin: hit_record.p,
            direction: &target - &hit_record.p
        }, hitable) * 0.5
    } else {
        let unit_direction : Vec3 = r.direction.unit_vector();
        let t              : f32  = 0.5 * (unit_direction.y + 1.0);
        &(&Color3 { r: 1.0, g: 1.0, b: 1.0 } * (1.0 - t)) + &(&Color3 {r: 0.5, g: 0.7, b: 1.0} * t)
    }
}

fn main() {
    let mut writer = io::BufWriter::new(io::stdout());

    let seed: [u8; 32] = [13; 32];
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);

    let nx: i32 = 200;
    let ny: i32 = 100;
    let ns: i32 = 100;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();

    let hitable           = ListHitable { hitables: vec![
        SphereHitable {center: Vec3{x: 0.0, y: 0.0, z: -1.0}, radius: 0.5},
        SphereHitable {center: Vec3{x: 0.0, y: -100.5, z: -1.0}, radius: 100.0},
    ]};
    let camera: Camera = Camera{};
    let mut j = ny - 1;
    while j >= 0 {
        for i in 0..nx {
            let mut col: Color3 = Color3 {r: 0.0, g: 0.0, b: 0.0};
            for _ in 0..ns {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r: Ray = camera.get_ray(u, v);
                col = &col + &color(&mut rng, &r, &hitable);
            }
            col = &col / ns as f32;
            col = Color3 {r: col.r.sqrt(), g: col.g.sqrt(), b: col.b.sqrt()};
            writer.write_all(format!("{} {} {}\n", col.ir(), col.ig(), col.ib()).as_bytes()).unwrap();
        }
        j -= 1;
    }
}
