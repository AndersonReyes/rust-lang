use crate::materials::{lambertian::Lambertian, result::MaterialResult};
use crate::math::ray::Ray;
use crate::math::{Normal3f, Point3f};

pub type MaterialFn =
    fn(ray: &Ray, normal: &Normal3f, hit_point: &Point3f) -> Option<MaterialResult>;

pub enum Material {
    Lambertian(Lambertian),
    Custom(MaterialFn),
}

impl Material {
    pub fn get_color(
        &self,
        ray: &Ray,
        normal: &Normal3f,
        hit_point: &Point3f,
    ) -> Option<MaterialResult> {
        match self {
            Material::Lambertian(l) => l.get_color(ray, normal, hit_point),
            Material::Custom(f) => f(ray, normal, hit_point),
        }
    }
}
