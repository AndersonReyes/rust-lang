mod image;
mod math;

use raytracer::{
    camera::Camera,
    geometry::{geometry::Geometry, sphere::Sphere},
    image::color,
    materials::{lambertian::Lambertian, material::Material},
    math::Point3f,
};

use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mat: Material = Material::Lambertian(Lambertian::new(color::BLUE));
    let mat2: Material = Material::Lambertian(Lambertian::new(color::GREEN));

    let camera = Camera::new(16.0 / 9.0, 1.0, 2.0, 400);
    println!(
        "Camera width and height: {} {}\n",
        camera.width, camera.height
    );

    let ground = Geometry::Sphere(Sphere::new(2.2, Point3f::new(0.0, -2.7, -2.0), &mat2));
    let sphere = Geometry::Sphere(Sphere::new(0.5, Point3f::new(0.0, 0.0, -1.0), &mat));

    let scene: Vec<Geometry> = vec![ground, sphere];

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("target/image.ppm")?;

    camera.render(&scene, &mut file)?;

    Ok(())
}
