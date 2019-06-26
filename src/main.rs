use std::io;
use std::io::Write;

mod color3;
mod vec3;
mod ray;

use color3::Color3;
use vec3::Vec3;
use ray::Ray;

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc: Vec3 = &r.origin - center;
    let a : f32  = r.direction.dot(&r.direction);
    let b : f32  = 2.0 * oc.dot(&r.direction);
    let c : f32  = oc.dot(&oc) - radius*radius;
    let discriminant: f32 = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn color(r: &Ray) -> Color3 {
    if hit_sphere(&Vec3{x: 0.0, y: 0.0, z: -1.0}, 0.5, r) {
        Color3{r: 1.0, g: 0.0, b: 0.0}
    } else {
        let unit_direction : Vec3 = r.direction.unit_vector();
        let t              : f32  = 0.5 * (unit_direction.y + 1.0);
        &(&Color3 { r: 1.0, g: 1.0, b: 1.0 } * (1.0 - t)) + &(&Color3 {r: 0.5, g: 0.7, b: 1.0} * t)
    }
}

fn main() {
    let mut writer = io::BufWriter::new(io::stdout());

    let nx: i32 = 200;
    let ny: i32 = 100;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();

    let lower_left_corner : Vec3 = Vec3{x: -2.0, y: -1.0, z: -1.0};
    let horizontal        : Vec3 = Vec3{x: 4.0, y: 0.0, z: 0.0};
    let vertical          : Vec3 = Vec3{x: 0.0, y: 2.0, z: 0.0};
    let origin            : Vec3 = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let mut j = ny - 1;
    while j >= 0 {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;
            let r: Ray = Ray{
                origin,
                direction: &(&lower_left_corner + &(&horizontal * u)) + &(&vertical * v)
            };

            let col: Color3 = color(&r);
            writer.write_all(format!("{} {} {}\n", col.ir(), col.ig(), col.ib()).as_bytes()).unwrap();
        }
        j -= 1;
    }
}
