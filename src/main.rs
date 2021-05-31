mod structs;

use indicatif::ProgressBar;
use std::fs::File;
use std::io::prelude::*;
use structs::Pixel;

fn main() -> std::io::Result<()> {
    write_img("test.ppm", 1980, 1080)?;
    Ok(())
}

fn write_img(path: &str, width: u32, height: u32) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let mut buf = String::new();
    buf.push_str("P3\n");
    buf.push_str(&format!("{} {}\n", width, height));
    buf.push_str("255\n");

    let bar = ProgressBar::new(height as u64);

    for x in 0..height {
        for y in 0..width {
            let pixel = Pixel::from_f32(
                y as f32 / width as f32,
                (height - x) as f32 / height as f32,
                0.25,
            );
            buf.push_str(&format!("{} ", pixel));
        }
        bar.inc(1);
        buf.push_str("\n");
    }

    file.write_all(buf.as_bytes())?;
    Ok(())
}
