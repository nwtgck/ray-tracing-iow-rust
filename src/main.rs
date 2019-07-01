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
mod material;
mod util;

use color3::Color3;
use vec3::Vec3;
use ray::Ray;
use hitable::Hitable;
use list_hitable::ListHitable;
use sphere_hitable::SphereHitable;
use material::{LambertMaterial, MetalMaterial};


use camera::Camera;
use crate::material::DielectricMaterial;

fn color<H: Hitable>(rng: &mut rand::rngs::StdRng, r: &Ray, hitable: &H, depth: i32) -> Color3 {
    if let Some(hit_record) = hitable.hit(r, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some(scatter_record) = hit_record.material.scatter(rng, r, &hit_record) {
                let col = color(rng, &scatter_record.scattered, hitable, depth+1);
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

fn random_scene(rng: &mut rand::rngs::StdRng) -> impl Hitable {
    let mut hitables: Vec<SphereHitable> = Vec::new();

    hitables.push(SphereHitable {
        center: Vec3{x: 0.0, y: -1000.0, z: 0.0},
        radius: 1000.0,
        material: Box::new(LambertMaterial{albedo: Color3{r: 0.5, g: 0.5, b: 0.5}})
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center: Vec3 = Vec3 {
                x: a as f32 + 0.9 * rng.gen::<f32>(),
                y: 0.2,
                z: b as f32 + 0.9 * rng.gen::<f32>()
            };

            if (&center - &Vec3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                if choose_mat < 0.8 {
                    hitables.push(SphereHitable {
                        center,
                        radius: 0.2,
                        material: Box::new(LambertMaterial{
                            albedo: Color3{
                                r: rng.gen::<f32>() * rng.gen::<f32>(),
                                g: rng.gen::<f32>() * rng.gen::<f32>(),
                                b: rng.gen::<f32>() * rng.gen::<f32>()
                            }
                        })
                    });
                } else if choose_mat < 0.95 {
                    hitables.push(SphereHitable {
                        center,
                        radius: 0.2,
                        material: Box::new(MetalMaterial{
                            albedo: Color3{
                                r: 0.5 * (1.0 + rng.gen::<f32>()),
                                g: 0.5 * (1.0 + rng.gen::<f32>()),
                                b: 0.5 * (1.0 + rng.gen::<f32>())
                            },
                            f: 0.5 * rng.gen::<f32>()
                        })
                    });
                } else {
                    hitables.push(SphereHitable {
                        center,
                        radius: 0.2,
                        material: Box::new(DielectricMaterial{ref_idx: 1.5})
                    });
                }
            }
        }
    }

    hitables.push(SphereHitable {
        center: Vec3{x: 0.0, y: 1.0, z: 0.0},
        radius: 1.0,
        material: Box::new(DielectricMaterial{ref_idx: 1.5})
    });
    hitables.push(SphereHitable {
        center: Vec3{x: -4.0, y: 1.0, z: 0.0},
        radius: 1.0,
        material: Box::new(LambertMaterial{albedo: Color3{r: 0.4, g: 0.2, b: 0.1}})
    });
    hitables.push(SphereHitable {
        center: Vec3{x: 4.0, y: 1.0, z: 0.0},
        radius: 1.0,
        material: Box::new(MetalMaterial{albedo: Color3{r: 0.7, g: 0.6, b: 0.5}, f: 0.0})
    });

    ListHitable{hitables}
}

use std::path::PathBuf;
use structopt::StructOpt;

/// Ray Tracing in One Weekend in Rust
#[derive(StructOpt, Debug)]
#[structopt(name = "ray-tracing-iow")]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// Image width
    #[structopt(long, default_value = "600")]
    width: u32,

    /// Image height
    #[structopt(long, default_value = "400")]
    height: u32,

    /// Number of samples
    #[structopt(long, default_value = "10")]
    n_samples: u32,

    /// Random seed
    #[structopt(long, default_value = "101")]
    random_seed: u8,

    /// Output file path
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    // Parse options
    let opt = Opt::from_args();

    let mut writer = io::BufWriter::new(io::stdout());

    let seed: [u8; 32] = [opt.random_seed; 32];
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);

    let nx: u32 = opt.width;
    let ny: u32 = opt.height;
    let ns: u32 = opt.n_samples;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();

    let hitable = random_scene(&mut rng);

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
    let mut j = ny - 1;
    while j >= 0 {
        for i in 0..nx {
            let mut col: Color3 = Color3 {r: 0.0, g: 0.0, b: 0.0};
            for _ in 0..ns {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r: Ray = camera.get_ray(&mut rng, u, v);
                col = &col + &color(&mut rng, &r, &hitable, 0);
            }
            col = &col / ns as f32;
            col = Color3 {r: col.r.sqrt(), g: col.g.sqrt(), b: col.b.sqrt()};
            writer.write_all(format!("{} {} {}\n", col.ir(), col.ig(), col.ib()).as_bytes()).unwrap();
        }
        j -= 1;
    }
}
