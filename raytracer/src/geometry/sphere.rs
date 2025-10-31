use crate::{
    geometry::intersectable::{HitRecord, Intersectable},
    interval::Interval,
    materials::material::{Material, MaterialResult},
    math::{Vector3f, ray::Ray},
};

pub struct Sphere<'a> {
    pub radius: f64,
    pub center: Vector3f,
    pub material: &'a Box<dyn Material>,
}

impl<'a> Sphere<'a> {
    pub fn new(radius: f64, center: Vector3f, material: &'a Box<dyn Material>) -> Self {
        Self {
            radius,
            center,
            material,
        }
    }
}

impl Intersectable for Sphere<'_> {
    fn intersect(&self, ray: &Ray, _interval: Interval) -> Option<HitRecord> {
        let oc: Vector3f = self.center - ray.origin;
        let aa = ray.direction.dot(&ray.direction);
        let bb = -2.0 * ray.direction.dot(&oc);
        let cc = oc.dot(&oc) - self.radius * self.radius;
        let dd = bb * bb - 4.0 * aa * cc;

        let intersection_point = ray.at(dd);
        let time = dd;
        let normal = (intersection_point - self.center) / self.radius;

        Some(dd).filter(|root| *root >= 0.0).map(|_| {
            let mat_result: MaterialResult = self
                .material
                .get_color(ray, &normal, &intersection_point)
                .expect("Failed to get color from material for sphere");

            HitRecord::new(
                intersection_point,
                normal,
                mat_result.attenuation,
                time,
                ray,
            )
        })
    }
}
