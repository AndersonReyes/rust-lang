use crate::{
    geometry::intersectable::{HitRecord, Intersectable},
    interval::Interval,
    math::ray::Ray,
};

pub struct IntersectableGroup {
    items: Vec<Box<dyn Intersectable>>,
}

impl Intersectable for IntersectableGroup {
    fn intersect(&self, ray: &Ray, interval: Interval) -> Option<super::intersectable::HitRecord> {
        let mut closest: Option<HitRecord> = None;
        let mut closest_time = interval.max;

        for obj in self.items.iter() {
            if let Some(hit) = obj.intersect(ray, Interval::new(interval.min, closest_time)) {
                closest_time = hit.time;
                closest = Some(hit);
            }
        }

        closest
    }
}

impl IntersectableGroup {
    pub fn new(items: Vec<Box<dyn Intersectable>>) -> Self {
        Self { items }
    }
}
