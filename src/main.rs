use std::io;
use std::fs;
use std::io::Write;
use rand;
use rand::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

mod color3;
mod vec3;
mod ray;
mod hitable;
mod list_hitable;
mod sphere_hitable;
mod camera;
mod material;
mod util;
mod render;

use color3::Color3;
use vec3::Vec3;
use hitable::Hitable;
use list_hitable::ListHitable;
use sphere_hitable::SphereHitable;
use material::{LambertMaterial, MetalMaterial};

use crate::material::DielectricMaterial;


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

    /// Minimum float number
    #[structopt(long, default_value = "0.001")]
    min_float: f32,

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

    // Select output destination whether file or stdout
    // (from: https://users.rust-lang.org/t/how-to-create-bufreader---from-option-file-with-std-io-stdout-as-fallback-in-a-rust-way/12980/2?u=nwtgck)
    let write: Box<Write> =
        if let Some(file_path) = opt.file {
            Box::new(fs::File::create(file_path).unwrap())
        } else {
            Box::new(io::stdout())
        };
    let writer = io::BufWriter::new(write);

    let seed: [u8; 32] = [opt.random_seed; 32];
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);

    let hitable = random_scene(&mut rng);

    render::render(
        writer,
        opt.random_seed,
        hitable,
        opt.width,
        opt.height,
        opt.n_samples,
        opt.min_float
    );
}
