mod image;
mod math;

extern crate nalgebra;

use raytracer::{
    geometry::{intersectable::Intersectable, sphere::Sphere},
    image::color::{BLUE, Color},
    interval::Interval,
    materials::{lambertian::Lambertian, material::Material},
    math::{Point3, Vector3f, ray::Ray},
};

use std::{
    error::Error,
    f64,
    fs::{File, OpenOptions},
    io::Write,
};

fn random_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn write_color_ppm(color: &Color, file: &mut File) -> std::io::Result<()> {
    file.write(
        format!(
            "{} {} {}\n",
            (255.999 * color.x).floor() as i32,
            (255.999 * color.y).floor() as i32,
            (255.999 * color.z).floor() as i32,
        )
        .as_bytes(),
    )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let aspect_ratio = 16.0 / 9.0;
    let image_width_int: i32 = 400;
    let image_width: f64 = image_width_int as f64;
    let image_height: f64 = (image_width / aspect_ratio).floor();
    let image_height_int: i32 = image_height as i32;

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height);
    let camera_center: Vector3f = Vector3f::new(0.0, 0.0, 0.0);

    let viewport_u: Vector3f = Vector3f::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vector3f = Vector3f::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u: Vector3f = viewport_u / image_width;
    let pixel_delta_v: Vector3f = viewport_v / image_height;

    let viewport_upper_left: Vector3f =
        camera_center - Vector3f::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let center_pixel_location: Vector3f =
        viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("target/image.ppm")?;

    file.write(format!("P3\n {} {}\n255\n", image_width, image_height).as_bytes())?;

    let mat: Box<dyn Material> = Box::new(Lambertian::new(BLUE));
    let sphere = Sphere::new(0.5, Point3::new(0.0, 0.0, -1.0), &mat);

    // TODO: this casting to i32 is ugly, improve later
    for j in 0..image_height_int {
        for i in 0..image_width_int {
            let pixel_center: Vector3f =
                center_pixel_location + (pixel_delta_u * (i as f64)) + (pixel_delta_v * (j as f64));
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = sphere
                .intersect(&ray, Interval::new(0.001, f64::INFINITY))
                .map(|hit| hit.color)
                .unwrap_or(random_color(&ray));

            write_color_ppm(&color, &mut file)?;
        }
    }

    Ok(())
}
