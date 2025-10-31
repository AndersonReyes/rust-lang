mod image;
mod math;

extern crate nalgebra;

use raytracer::{
    camera::Camera,
    display::Display,
    geometry::{intersectable_group::IntersectableGroup, sphere::Sphere},
    image::color::{BLUE, Color},
    materials::{lambertian::Lambertian, material::Material},
    math::{Point3, ray::Ray},
};

use std::error::Error;

use crate::image::color::BLACK;

fn random_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

//fn write_color_ppm(color: &Color, file: &mut File) -> std::io::Result<()> {
//    file.write(format!("{} {} {}\n", as_u8(color.x), as_u8(color.y), as_u8(color.z),).as_bytes())?;
//    Ok(())
//}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mat: Box<dyn Material> = Box::new(Lambertian::new(BLUE));

    let mut camera = Camera::new(16.0 / 9.0, 1.0, 2.0, 800);

    let mut display = Display::new(camera.width, camera.height);

    let mut color_data: Vec<Vec<Color>> =
        vec![vec![BLACK; camera.height as usize]; camera.width as usize];

    let scene = IntersectableGroup::new(vec![Box::new(Sphere::new(
        0.5,
        Point3::new(0.0, 0.0, -1.0),
        &mat,
    ))]);

    while camera.is_running && display.is_running {
        display.handle_quit();
        camera.ray_step(&scene, &mut color_data);
        display.update(&color_data);
    }

    display.wait_until_quit();

    Ok(())
}
