use crate::geometry::intersectable::Intersectable;
pub use crate::shapes::sphere::Sphere;

pub enum Shapes {
    Sphere(Sphere),
}

impl Intersectable for Shapes {
    fn intersect(
        &self,
        ray: &crate::math::Ray,
        interval: crate::interval::Interval,
    ) -> Option<crate::geometry::intersectable::HitRecord> {
        match self {
            Shapes::Sphere(s) => s.intersect(ray, interval),
        }
    }
}
