#![allow(dead_code)]

mod image;
mod interval;
mod math;

use raytracer::{
    camera::Camera,
    image::color,
    math::Vector3f,
    shapes::{shapes::Shapes, sphere::Sphere},
};

use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 1024;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("target/image.ppm")?;

    let sphere = Sphere::new(0.5, Vector3f::new(0.0, 0.0, -1.0), color::RED);
    let ground = Sphere::new(100.0, Vector3f::new(0.0, -110.5, -10.0), color::GREEN);

    let camera = Camera::new(aspect_ratio, image_width);
    camera.render(
        &mut file,
        &vec![Shapes::Sphere(sphere), Shapes::Sphere(ground)],
    )?;
    Ok(())
}
