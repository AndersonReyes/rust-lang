mod image;
mod math;

extern crate nalgebra;

use raytracer::{
    camera::Camera,
    display::Display,
    geometry::{geometry::Geometry, sphere::Sphere},
    image::color::{BLUE, Color},
    materials::{lambertian::Lambertian, material::Material},
    math::Point3,
};

use std::error::Error;

use crate::image::color::BLACK;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mat: Material = Material::Lambertian(Lambertian::new(BLUE));

    let mut camera = Camera::new(16.0 / 9.0, 1.0, 2.0, 400);

    let mut display = Display::new(camera.width, camera.height);

    let mut color_data: Vec<Vec<Color>> =
        vec![vec![BLACK; camera.height as usize]; camera.width as usize];

    let scene: Vec<Geometry> = vec![Geometry::Sphere(Sphere::new(
        0.5,
        Point3::new(0.0, 0.0, -1.0),
        &mat,
    ))];

    while camera.is_running && display.is_running {
        display.handle_quit();
        camera.ray_step(&scene, &mut color_data);
        display.update(&color_data);
    }

    display.wait_until_quit();

    Ok(())
}
