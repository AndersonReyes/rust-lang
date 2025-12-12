use crate::math::utils;
use crate::math::{Normal3f, Point3f, Ray, Vector3f};
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
        _ray: &Ray,
        normal: &Normal3f,
        hit_point: &Point3f,
    ) -> Option<MaterialResult> {
        let mut direction = Vector3f::random_unit_vector() + *normal;

        if utils::near_zero_vector(&direction) {
            direction = normal.to_owned();
        }

        Some(MaterialResult::new(
            self.albedo,
            Ray::new(hit_point.to_owned(), direction),
        ))
    }
}
