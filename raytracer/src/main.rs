mod image;
extern crate nalgebra;

use image::color::Color;

use std::{error::Error, fs::File, fs::OpenOptions, io::Write};

fn random_color(image_height: i32, image_width: i32, i: i32, j: i32) -> Color {
    Color::new(
        (255.999 * f64::from(i) / f64::from(image_width - 1)) as i64,
        (255.999 * f64::from(j) / f64::from(image_height - 1)) as i64,
        128,
    )
}

fn write_color(color: &Color, file: &mut File) -> std::io::Result<()> {
    file.write(format!("{} {} {}\n", color.x, color.y, color.z).as_bytes())?;
    Ok(())
}

fn write_ppm(image_height: i32, image_width: i32) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("target/image.ppm")?;

    file.write(format!("P3\n {} {}\n255\n", image_width, image_height).as_bytes())?;

    for j in 0..image_height {
        for i in 0..image_width {
            let color = random_color(image_height, image_width, i, j);

            write_color(&color, &mut file)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    write_ppm(256, 256)?;

    Ok(())
}
