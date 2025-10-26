use crate::geometry::intersectable::HitRecord;
use crate::image::color::Color;
use crate::math::ray::Ray;

pub trait Material {
    fn get_color(ray: &Ray, hit_record: &HitRecord) -> Option<Color>;
}
