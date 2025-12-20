use crate::{
    geometry::intersectable::{HitRecord, Intersectable},
    image::color,
    math::Vector3f,
};

pub struct Sphere {
    radius: f64,
    position: Vector3f,
}

impl Sphere {
    pub fn new(radius: f64, position: Vector3f) -> Self {
        Self { radius, position }
    }
}

impl Intersectable for Sphere {
    fn intersect(
        &self,
        ray: &crate::math::Ray,
        interval: crate::interval::Interval,
    ) -> Option<crate::geometry::intersectable::HitRecord> {
        let oc = self.position - ray.origin;
        let a = ray.direction.norm_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;
        let mut discriminant: f64 = h * h - a * c;

        if discriminant >= 0.0 {
            discriminant = discriminant.sqrt();
            let mut root = (h - discriminant) / a;
            if !interval.surrounds(root) {
                root = (h + discriminant) / a;

                if !interval.surrounds(root) {
                    return None;
                }
            }

            let hit_point = ray.at(root);
            let normal = (hit_point - self.position) / self.radius;

            Some(HitRecord::new(
                hit_point,
                normal,
                color::RED,
                root,
                ray.clone().into(),
            ))
        } else {
            None
        }
    }
}
