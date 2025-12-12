use crate::{
    geometry::intersectable::{HitRecord, Intersectable},
    interval::Interval,
    materials::{material::Material, result::MaterialResult},
    math::{Vector3f, ray::Ray},
};

pub struct Sphere<'a> {
    pub radius: f64,
    pub center: Vector3f,
    pub material: &'a Material,
}

impl<'a> Sphere<'a> {
    pub fn new(radius: f64, center: Vector3f, material: &'a Material) -> Self {
        Self {
            radius,
            center,
            material,
        }
    }
}

impl Intersectable for Sphere<'_> {
    fn intersect(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc: Vector3f = self.center - ray.origin;
        let a = ray.direction.norm_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.norm_squared() - (self.radius * self.radius);

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);
        let mut root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let time = root;
        let intersection_point = ray.at(time);
        let mut normal = (intersection_point - self.center) / self.radius;

        if ray.direction.dot(&normal) >= 0.0 {
            normal = -normal;
        }

        let mat_result: MaterialResult = self
            .material
            .get_color(ray, &normal, &intersection_point)
            .expect("Failed to get color from material for sphere");

        Some(HitRecord::new(
            intersection_point,
            normal,
            mat_result.attenuation,
            time,
            mat_result.scattered_ray,
        ))
    }
}
