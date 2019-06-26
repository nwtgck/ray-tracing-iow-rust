use std::io;
use std::io::Write;

mod color3;
mod vec3;
mod ray;
mod hitable;
mod list_hitable;
mod sphere_hitable;

use color3::Color3;
use vec3::Vec3;
use ray::Ray;
use crate::hitable::Hitable;
use crate::list_hitable::ListHitable;
use crate::sphere_hitable::SphereHitable;

fn color<H: Hitable>(r: &Ray, hitable: &H) -> Color3 {
    if let Some(hit_record) = hitable.hit(r, 0.0, std::f32::MAX) {
        &Color3 {
            r: hit_record.normal.x + 1.0,
            g: hit_record.normal.y + 1.0,
            b: hit_record.normal.z + 1.0,
        } * 0.5
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
    let hitable           = ListHitable { hitables: vec![
        SphereHitable {center: Vec3{x: 0.0, y: 0.0, z: -1.0}, radius: 0.5},
        SphereHitable {center: Vec3{x: 0.0, y: -100.5, z: -1.0}, radius: 100.0},
    ]};
    let mut j = ny - 1;
    while j >= 0 {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;
            let r: Ray = Ray{
                origin,
                direction: &(&lower_left_corner + &(&horizontal * u)) + &(&vertical * v)
            };

            let col: Color3 = color(&r, &hitable);
            writer.write_all(format!("{} {} {}\n", col.ir(), col.ig(), col.ib()).as_bytes()).unwrap();
        }
        j -= 1;
    }
}
