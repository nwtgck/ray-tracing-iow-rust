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

    /// Animation output directory
    #[structopt(long)]
    anime_out_dir_path: Option<String>,

    /// Animation dt
    #[structopt(long, default_value = "0.005")]
    anime_dt: f32,

    /// Animation minimum time
    #[structopt(long, default_value = "0.0")]
    anime_min_t: f32,

    /// Animation max time
    #[structopt(long, default_value = "6.0")]
    anime_max_t: f32,

    /// Animation max time
    #[structopt(long, default_value = "6")]
    anime_skip_step: usize,

    /// Output file path
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    // Parse options
    let opt = Opt::from_args();

    // If render animation
    if let Some(anime_out_dir_path_str) = opt.anime_out_dir_path {
        // Get animation output directory path
        let anime_out_dir_path = std::path::Path::new(&anime_out_dir_path_str);
        // Get scene iterator
        let scene_iter: random_scenes::FreeFallAnimation = random_scenes::FreeFallAnimation::new(opt.width, opt.height, opt.anime_dt, opt.anime_min_t, opt.anime_max_t, opt.random_seed);
        // Skip by step
        let scene_iter= util::skip_by_step(scene_iter, opt.anime_skip_step);
        // Render animation frame by frame
        render::render_animation(anime_out_dir_path, opt.random_seed, scene_iter, opt.width, opt.height, opt.n_samples, opt.min_float);
    } else {
        // Select output destination whether file or stdout
        // (from: https://users.rust-lang.org/t/how-to-create-bufreader---from-option-file-with-std-io-stdout-as-fallback-in-a-rust-way/12980/2?u=nwtgck)
        let write: Box<Write> =
            if let Some(file_path) = opt.file {
                Box::new(fs::File::create(file_path).unwrap())
            } else {
                Box::new(io::stdout())
            };
        let writer = io::BufWriter::new(write);

        // Get random generator
        let mut rng: rand::rngs::StdRng = util::rng_by_seed(opt.random_seed);

        // Generate scene
        let scene = random_scenes::iow_book_cover(&mut rng, opt.width, opt.height);

        // Render by ray tracing
        render::render(
            writer,
            opt.random_seed,
            &scene,
            opt.width,
            opt.height,
            opt.n_samples,
            opt.min_float
        );
    }
}
