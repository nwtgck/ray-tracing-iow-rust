use std::io;
use std::io::Write;

mod color3;
mod vec3;
mod ray;

use color3::Color3;
use vec3::Vec3;
use ray::Ray;

fn color(r: &Ray) -> Color3 {
    let unit_direction : Vec3 = r.direction.unit_vector();
    let t              : f32  = 0.5f32 * (unit_direction.y + 1.0f32);
    Color3 { r: 1f32, g: 1f32, b: 1f32 } * (1f32 - t) + Color3 {r: 0.5f32, g: 0.7f32, b: 1.0f32} * t
}

fn main() {
    let mut writer = io::BufWriter::new(io::stdout());

    let nx: i32 = 200;
    let ny: i32 = 100;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();

    let lower_left_corner : Vec3 = Vec3{x: -2.0f32, y: -1.0f32, z: -1.0f32};
    let horizontal        : Vec3 = Vec3{x: 4.0f32, y: 0.0f32, z: 0.0f32};
    let vertical          : Vec3 = Vec3{x: 0.0f32, y: 2.0f32, z: 0.0f32};
    let origin            : Vec3 = Vec3{x: 0.0f32, y: 0.0f32, z: 0.0f32};
    let mut j = ny - 1;
    while j >= 0 {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;
            let r: Ray = Ray{
                origin,
                direction: lower_left_corner + horizontal * u + vertical * v
            };

            let col: Color3 = color(&r);
            writer.write_all(format!("{} {} {}\n", col.ir(), col.ig(), col.ib()).as_bytes()).unwrap();
        }
        j -= 1;
    }
}
