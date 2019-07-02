use std::io;
use std::fs;
use std::io::Write;
use rand;
use std::path::PathBuf;
use structopt::StructOpt;
use crate::scene::Scene;
use core::borrow::Borrow;

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
mod random_scenes;
mod scene;


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

    // TODO: Hard code
    let a: random_scenes::FreeFallAnimation = random_scenes::FreeFallAnimation::new(opt.width, opt.height, 0.005, 0.0, 6.0, 3);
//    let first_scene = a.collect::<Vec<_>>().pop().unwrap();

    for (idx, scene) in a.enumerate() {
        // TODO: Hard code: path
        let file_path = format!("anime/anime{:08}.ppm", idx + 1);
        println!("{}", file_path);
        let writer = io::BufWriter::new(fs::File::create(file_path).unwrap());
        // Render by ray tracing
        render::render(
            writer,
            3, // TODO: Hard code: seed
            scene,
            opt.width,
            opt.height,
            opt.n_samples,
            opt.min_float
        );
    }



    println!("HERE!");
    // TODO: remove
    std::process::exit(0);


//    // Select output destination whether file or stdout
//    // (from: https://users.rust-lang.org/t/how-to-create-bufreader---from-option-file-with-std-io-stdout-as-fallback-in-a-rust-way/12980/2?u=nwtgck)
//    let write: Box<Write> =
//        if let Some(file_path) = opt.file {
//            Box::new(fs::File::create(file_path).unwrap())
//        } else {
//            Box::new(io::stdout())
//        };
//    let writer = io::BufWriter::new(write);
//
//    // Get random generator
//    let mut rng: rand::rngs::StdRng = util::rng_by_seed(opt.random_seed);
//
//    // Generate scene
//    let scene = random_scenes::iow_book_cover(&mut rng, opt.width, opt.height);
//
//    // Render by ray tracing
//    render::render(
//        writer,
//        opt.random_seed,
//        first_scene,
////        scene,
//        opt.width,
//        opt.height,
//        opt.n_samples,
//        opt.min_float
//    );
}
