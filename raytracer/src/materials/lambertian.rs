use crate::math::{Normal3, Point3};
use crate::math::{ray::Ray, utils};
use crate::{image::color::Color, materials::result::MaterialResult};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn get_color(
        &self,
        ray: &Ray,
        normal: &Normal3,
        hit_point: &Point3,
    ) -> Option<MaterialResult> {
        let mut direction = normal + utils::random_unit_vector();

        if utils::near_zero_vector(&direction) {
            direction = normal.clone_owned();
        }

        Some(MaterialResult::new(
            self.albedo,
            Ray::new(hit_point.clone_owned(), direction),
        ))
    }
}
