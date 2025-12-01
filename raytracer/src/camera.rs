use std::{fs::File, io::Write};

use crate::{
    geometry::{geometry::Geometry, intersectable},
    image::color::{Color, as_u8},
    interval::Interval,
    math::{Vector3f, ray::Ray},
};

pub struct Camera {
    camera_center: Vector3f,
    center_pixel_location: Vector3f,
    pixel_delta_u: Vector3f,
    pixel_delta_v: Vector3f,
    viewport_u: Vector3f,
    viewport_v: Vector3f,
    viewport_w: Vector3f,
    pub width: u32,
    pub height: u32,
    step_width: u32,
    step_height: u32,
    pub is_running: bool,
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
        let center_pixel_location: Vector3f = viewport_w + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            camera_center,
            center_pixel_location,
            pixel_delta_u,
            pixel_delta_v,
            viewport_u,
            viewport_v,
            viewport_w,
            width: image_width_int,
            height: image_height_int,
            step_width: 0,
            step_height: 0,
            is_running: true,
        }
    }

    // takes a one pixel step and sends a ray into the screen.
    pub fn render(&mut self, scene: &Vec<Geometry>, file: &mut File) -> std::io::Result<()> {
        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_center: Vector3f = self.center_pixel_location
                    + (self.pixel_delta_u * (x as f64))
                    + (self.pixel_delta_v * (y as f64));

                let ray_direction = pixel_center - self.camera_center;
                let ray = Ray::new(self.camera_center, ray_direction);
                let color =
                    intersectable::intersect(&scene, &ray, Interval::new(0.001, f64::INFINITY))
                        .map(|hit| hit.color)
                        .unwrap_or(random_color(&ray));

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

    // takes a one pixel step and sends a ray into the screen.
    pub fn ray_step(&mut self, scene: &Vec<Geometry>, color_data: &mut Vec<Vec<Color>>) {
        if !self.is_running {
            return;
        }

        let pixel_center: Vector3f = self.center_pixel_location
            + (self.pixel_delta_u * (self.step_width as f64))
            + (self.pixel_delta_v * (self.step_height as f64));

        let ray_direction = pixel_center - self.camera_center;
        let ray = Ray::new(self.camera_center, ray_direction);
        let color = intersectable::intersect(&scene, &ray, Interval::new(0.001, f64::INFINITY))
            .map(|hit| hit.color)
            .unwrap_or(random_color(&ray));

        // write_color_ppm(&color, &mut file)?;
        color_data[self.step_height as usize][self.step_width as usize] = color;

        self.step_width += 1;
        self.step_height += 1;

        if self.step_width >= self.width || self.step_height >= self.height {
            self.is_running = false;
        }
    }
}
