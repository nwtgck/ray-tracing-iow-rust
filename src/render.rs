use std::io;
use std::io::Write;
use rand::prelude::*;
use rayon::prelude::*;

use crate::color3::Color3;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::Hitable;
use crate::util;
use core::borrow::{BorrowMut, Borrow};
use crate::scene::Scene;

fn color(rng: &mut rand::rngs::StdRng, r: &Ray, hitable: &Hitable, min_float: f32, depth: i32) -> Color3 {
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

pub fn render<W: Write>(mut writer: io::BufWriter<W>, random_seed: u8, scene: Scene, width: u32, height: u32, n_samples: u32, min_float: f32) {
    let mut rng = util::rng_by_seed(random_seed);

    let nx: u32 = width;
    let ny: u32 = height;
    let ns: u32 = n_samples;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();

    let camera = scene.camera;
//    let hitable = scene.hitable;
    let h: &(Hitable + Sync) = scene.hitable.borrow();

    // Position and seed pairs
    let pos_and_seeds: Vec<((u32, u32), u8)> = {
        // NOTE: This random seed generation should be evaluated strictly for reproducible random
        let mut v: Vec<((u32, u32), u8)> = Vec::new();
        for j in (0..ny).rev() {
            for i in 0..nx {
                let seed: u8 = rng.gen();
                v.push(((i, j), seed));
            }
        }
        v
    };

    // Generate colors in pixels by ray tracing
    let colors: Vec<Color3> = pos_and_seeds.par_iter().cloned().map(|((i, j), seed) | {
        // Generate seeds
        let seeds: Vec<u8> = {
            let mut v: Vec<u8> = Vec::new();
            let mut rng = util::rng_by_seed(seed);
            for _ in 0..ns {
                v.push(rng.gen());
            }
            v
        };
        let mut col = seeds.par_iter()
            .map(|&seed| {
                let mut rng = util::rng_by_seed(seed);
                let u: f32 = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r: Ray = camera.get_ray(&mut rng, u, v);
                color(rng.borrow_mut(), &r, h, min_float, 0)
            })
            .reduce(|| Color3 {r: 0.0, g: 0.0, b: 0.0}, |sum, c| {
                &sum + &c
            });
        col = &col / ns as f32;
        col = Color3 {r: col.r.sqrt(), g: col.g.sqrt(), b: col.b.sqrt()};
        col
    }).collect();

    // Write the image pixels synchronously
    for col in colors {
        writer.write_all(format!("{} {} {}\n", col.ir(), col.ig(), col.ib()).as_bytes()).unwrap();
    }
}
