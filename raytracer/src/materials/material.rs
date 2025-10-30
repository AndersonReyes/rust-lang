use crate::image::color::Color;
use crate::math::ray::Ray;
use crate::math::{Normal3, Point3};

pub struct MaterialResult {
    pub attenuation: Color,
    pub scattered_ray: Ray,
}

impl MaterialResult {
    pub fn new(attenuation: Color, scattered_ray: Ray) -> MaterialResult {
        Self {
            attenuation: attenuation,
            scattered_ray: scattered_ray,
        }
    }
}

pub trait Material {
    fn get_color(&self, ray: &Ray, normal: &Normal3, hit_point: &Point3) -> Option<MaterialResult>;
}
