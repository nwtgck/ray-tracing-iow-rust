use std::io;
use std::io::Write;

mod color3;
mod vec3;

use color3::Color3;
use vec3::Vec3;

fn main() {
    let mut writer = io::BufWriter::new(io::stdout());

    let nx: i32 = 200;
    let ny: i32 = 100;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();
    let mut j = ny - 1;
    while j >= 0 {
        for i in 0..nx {
            let col: Color3 = Color3 {
                r: i as f32 / nx as f32,
                g: j as f32 / ny as f32,
                b: 0.2f32
            };
            writer.write_all(format!("{} {} {}\n", col.ir(), col.ig(), col.ib()).as_bytes()).unwrap();
        }
        j -= 1;
    }
}
