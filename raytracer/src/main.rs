mod image;
mod math;

use raytracer::{
    image::{Color, as_u8},
    math::Point3f,
};

use std::{fs::OpenOptions, io::Write};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let image_height: u16 = 1024;
    let image_width: u16 = 1024;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("target/image.ppm")?;

    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;

    for j in 0..image_height {
        for i in 0..image_width {
            let red: f64 = f64::from(i) / f64::from(image_width - 1);
            let green: f64 = f64::from(j) / f64::from(image_height - 1);

            let color = Color::new(red, green, 0.);
            // println!("Color: {:?}", color);
            write!(
                file,
                "{} {} {}\n",
                as_u8(color.x),
                as_u8(color.y),
                as_u8(color.z)
            )?;
        }
    }

    file.flush()?;

    Ok(())
}
