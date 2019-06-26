use std::io;
use std::io::Write;

fn main() {
    let mut writer = io::BufWriter::new(io::stdout());

    let nx: i32 = 200;
    let ny: i32 = 100;
    writer.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).unwrap();
    let mut j = ny - 1;
    while j >= 0 {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2f32;
            let color_elem_to_int = |f: f32| (255.99f32 * f) as i32;
            let ir: i32 = color_elem_to_int(r);
            let ig: i32 = color_elem_to_int(g);
            let ib: i32 = color_elem_to_int(b);
            writer.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).unwrap();
        }
        j -= 1;
    }
}
