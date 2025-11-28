use crate::image::color::Color;
use crate::math::ray::Ray;

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
