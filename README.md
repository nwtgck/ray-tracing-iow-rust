# ray-tracing-iow
[![CircleCI](https://circleci.com/gh/nwtgck/ray-tracing-iow-rust.svg?style=shield)](https://circleci.com/gh/nwtgck/ray-tracing-iow-rust)

Ray Tracing in One Weekend written in Rust

![Ray Tracing Animation](doc_assets/ray-tracing-animation.gif)

## Features
* Written in Rust
* Parallel processing by [Rayon](https://github.com/rayon-rs/rayon)
* Animation
* Reproducible random generation

## Usage

### One image generation

Create one image.

```bash
cargo run --release -- image.ppm
```

### Animation generation

Here is very small video generation.

```bash
# Generate .ppm files
cargo run --release -- --width=60 --height=40 --anime-out-dir-path=my_anime
# Create anime.mp4
cd my_anime
ffmpeg -i anime%08d.ppm -c:v libx264 -vf fps=25 -pix_fmt yuv420p anime.mp4
```

### Other options

```txt
ray-tracing-iow 0.1.0
Ryo Ota <nwtgck@gmail.com>
Ray Tracing in One Weekend in Rust

USAGE:
    ray-tracing-iow [OPTIONS] [file]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --anime-dt <anime-dt>                        Animation dt [default: 0.03]
        --anime-max-t <anime-max-t>                  Animation max time [default: 6.0]
        --anime-min-t <anime-min-t>                  Animation minimum time [default: 0.0]
        --anime-out-dir-path <anime-out-dir-path>    Animation output directory
        --height <height>                            Image height [default: 400]
        --min-float <min-float>                      Minimum float number [default: 0.001]
        --n-samples <n-samples>                      Number of samples [default: 10]
        --random-seed <random-seed>                  Random seed [default: 101]
        --width <width>                              Image width [default: 600]

ARGS:
    <file>    Output file path
```

## Related projects

Here are related projects.

* [iyahoo/clj-ray-tracing](https://github.com/iyahoo/clj-ray-tracing) (Clojure)
* [petershirley/raytracinginoneweekend](https://github.com/petershirley/raytracinginoneweekend) (C++)
* [nwtgck/ray-tracing-iow-scala](https://github.com/nwtgck/ray-tracing-iow-scala) (Scala)
