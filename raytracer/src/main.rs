mod image;
mod math;

use raytracer::{
    image::{Color, ppm},
    math::{Ray, Vector3f},
};

use std::{fs::OpenOptions, io::Write};

fn ray_color(ray: &Ray) -> Color<f64> {
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (Color::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Color::new(0.5, 0.7, 1.0) * a)
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 1024;
    let image_height: u16 = (f32::from(image_width) / aspect_ratio).max(1.0) as u16;

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
    let camera_center = Vector3f::new(0.0, 0., 0.);
    let viewport_u = Vector3f::new(viewport_width, 0., 0.);
    let viewport_v = Vector3f::new(0., -viewport_height, 0.);
    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / f64::from(image_width);
    let pixel_delta_v = viewport_v / f64::from(image_height);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vector3f::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("target/image.ppm")?;

    ppm::write_header(image_width, image_height, &mut file)?;

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * f64::from(i)) + (pixel_delta_v * f64::from(j));

            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = ray_color(&ray);
            //let red: f64 = f64::from(i) / f64::from(image_width - 1);
            //let green: f64 = f64::from(j) / f64::from(image_height - 1);

            //let color = Color::new(red, green, red + green);
            ppm::write_color(&color.as_u8(), &mut file)?;
        }
    }

    file.flush()?;

    Ok(())
}
