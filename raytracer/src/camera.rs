use std::{fs::File, io::Write};

use crate::{
    geometry::{geometry::Geometry, intersectable},
    image::{BLACK, Color, as_u8},
    interval::Interval,
    math::{Ray, Vector3f, utils},
};

pub struct Camera {
    camera_center: Vector3f,
    center_pixel_location: Vector3f,
    pixel_delta_u: Vector3f,
    pixel_delta_v: Vector3f,
    pub width: u32,
    pub height: u32,
    samples_per_pixel: u16,
}

fn random_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        focal_length: f64,
        viewport_height: f64,
        image_width_int: u32,
    ) -> Self {
        let image_width: f64 = image_width_int as f64;
        let image_height: f64 = (image_width / aspect_ratio).floor();
        let image_height_int: u32 = image_height as u32;

        // camera
        let viewport_width = viewport_height * (image_width / image_height);
        let camera_center: Vector3f = Vector3f::new(0.0, 0.0, 0.0);

        let viewport_u: Vector3f = Vector3f::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vector3f = Vector3f::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u: Vector3f = viewport_u / image_width;
        let pixel_delta_v: Vector3f = viewport_v / image_height;

        let viewport_w: Vector3f = camera_center
            - Vector3f::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let center_pixel_location: Vector3f = (viewport_w + 0.5) * (pixel_delta_u + pixel_delta_v);

        Self {
            camera_center,
            center_pixel_location,
            pixel_delta_u,
            pixel_delta_v,
            width: image_width_int,
            height: image_height_int,
            samples_per_pixel: 100,
        }
    }

    fn get_color(
        &self,
        depth: usize,
        scene: &Vec<Geometry>,
        ray: &Ray,
        interval: Interval,
    ) -> Color {
        if depth <= 0 {
            return BLACK;
        }

        if let Some(hit) = intersectable::intersect(&scene, &ray, interval.clone()) {
            return hit.color * self.get_color(depth - 1, scene, &hit.ray, interval.clone());
        } else {
            random_color(&ray)
        }
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = Vector3f::new(
            utils::random_double() - 0.5,
            utils::random_double() - 0.5,
            0.,
        );

        let pixel_sample = self.center_pixel_location
            + (self.pixel_delta_u * (offset.x + f64::from(x)))
            + (self.pixel_delta_v * (offset.y + f64::from(y)));

        let ray_direction = pixel_sample - self.camera_center;

        return Ray::new(self.camera_center, ray_direction);
    }

    // takes a one pixel step and sends a ray into the screen.
    pub fn render(&self, scene: &Vec<Geometry>, file: &mut File) -> std::io::Result<()> {
        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;

        for y in 0..self.height {
            for x in 0..self.width {
                let mut color = BLACK;

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color = color
                        + self.get_color(100, &scene, &ray, Interval::new(0.001, f64::INFINITY));
                }

                write!(
                    file,
                    "{} {} {}\n",
                    as_u8(color.x),
                    as_u8(color.y),
                    as_u8(color.z)
                )?;
            }

            file.flush()?;
        }

        file.flush()?;
        Ok(())
    }
}
