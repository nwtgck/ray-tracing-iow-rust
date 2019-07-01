use std::io;
use std::io::Write;
use rand::prelude::*;

use crate::color3::Color3;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::Hitable;
use crate::camera::Camera;

fn color<H: Hitable>(rng: &mut rand::rngs::StdRng, r: &Ray, hitable: &H, min_float: f32, depth: i32) -> Color3 {
    if let Some(hit_record) = hitable.hit(r, min_float, std::f32::MAX) {
        if depth < 50 {
            if let Some(scatter_record) = hit_record.material.scatter(rng, r, &hit_record) {
                let col = color(rng, &scatter_record.scattered, hitable, min_float, depth+1);
                let attenuation = scatter_record.attenuation;
                Color3 {
                    r: col.r * attenuation.r,
                    g: col.g * attenuation.g,
                    b: col.b * attenuation.b
                }
            } else {
                Color3 {r: 0.0, g: 0.0, b: 0.0}
            }
        } else {
            Color3 {r: 0.0, g: 0.0, b: 0.0}
        }
    } else {
        let unit_direction : Vec3 = r.direction.unit_vector();
        let t              : f32  = 0.5 * (unit_direction.y + 1.0);
        &(&Color3 { r: 1.0, g: 1.0, b: 1.0 } * (1.0 - t)) + &(&Color3 {r: 0.5, g: 0.7, b: 1.0} * t)
    }
}

pub fn render<W: Write, H: Hitable>(mut writer: io::BufWriter<W>, random_seed: u8, hitable: H, width: u32, height: u32, n_samples: u32, min_float: f32) {
    let seed: [u8; 32] = [random_seed; 32];
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);

    let nx: u32 = width;
    let ny: u32 = height;
    let ns: u32 = n_samples;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();

    let lookfrom: Vec3 = Vec3 {x: 13.0, y: 2.0, z: 3.0};
    let lookat  : Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
    let focus_dist: f32 = 10.0;
    let aperture  : f32  = 0.1;
    let camera: Camera = Camera{
        lookfrom,
        lookat,
        vup: Vec3 {x: 0.0, y: 1.0, z: 0.0},
        vfov: 20.0,
        aspect: nx as f32 / ny as f32,
        aperture,
        focus_dist
    };
    let mut j = (ny - 1) as i32;
    while j >= 0 {
        for i in 0..nx {
            let mut col: Color3 = Color3 {r: 0.0, g: 0.0, b: 0.0};
            for _ in 0..ns {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r: Ray = camera.get_ray(&mut rng, u, v);
                col = &col + &color(&mut rng, &r, &hitable, min_float, 0);
            }
            col = &col / ns as f32;
            col = Color3 {r: col.r.sqrt(), g: col.g.sqrt(), b: col.b.sqrt()};
            writer.write_all(format!("{} {} {}\n", col.ir(), col.ig(), col.ib()).as_bytes()).unwrap();
        }
        j -= 1;
    }
}
