use std::{fs::File, io};

use crate::{
    geometry::intersectable::Intersectable,
    image::{Color, ppm},
    interval::Interval,
    math::{Ray, Vector3f},
    shapes::shapes::Shapes,
};
use std::io::Write;

fn ray_color(ray: &Ray) -> Color<f64> {
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (Color::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Color::new(0.5, 0.7, 1.0) * a)
}

pub struct Camera {
    pixel00_loc: Vector3f,
    position: Vector3f,
    image_height: u16,
    image_width: u16,
    pixel_delta_u: Vector3f,
    pixel_delta_v: Vector3f,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u16) -> Self {
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
        let viewport_upper_left = camera_center
            - Vector3f::new(0., 0., focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            pixel00_loc,
            position: camera_center,
            image_height,
            image_width,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, file: &mut File, scene: &Vec<Shapes>) -> io::Result<()> {
        ppm::write_header(self.image_width, self.image_height, file)?;
        let interval = Interval::new(0.0, f64::MAX);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * f64::from(i))
                    + (self.pixel_delta_v * f64::from(j));

                let ray_direction = pixel_center - self.position;
                let ray = Ray::new(self.position, ray_direction);

                let mut closest_so_far = interval.max;
                let mut color = ray_color(&ray);

                for obj in scene.iter() {
                    if let Some(hit_record) =
                        obj.intersect(&ray, Interval::new(interval.min, closest_so_far))
                    {
                        closest_so_far = hit_record.time;
                        color = hit_record.color
                    };
                }

                ppm::write_color(&color.as_u8(), file)?;
            }
        }

        file.flush()?;

        Ok(())
    }
}
