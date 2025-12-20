use crate::{
    geometry::intersectable::{HitRecord, Intersectable},
    image::color::Colorf64,
    math::Vector3f,
};

pub struct Sphere {
    radius: f64,
    position: Vector3f,
    color: Colorf64,
}

impl Sphere {
    pub fn new(radius: f64, position: Vector3f, color: Colorf64) -> Self {
        Self {
            radius,
            position,
            color,
        }
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
                self.color.clone() * 0.5 + normal,
                root,
                ray.clone().into(),
            ))
        } else {
            None
        }
    }
}
