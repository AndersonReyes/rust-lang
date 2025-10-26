use crate::geometry::intersectable::HitRecord;
use crate::math::ray::Ray;
use crate::{image::color::Color, materials::material::Material};

pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn get_color(ray: &Ray, hit_record: &HitRecord) -> Option<Color> {
        let direction = hit_record.normal + random_unit_vector();
    }
}
