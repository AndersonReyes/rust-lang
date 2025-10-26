use crate::image::color::Color;
use crate::interval::Interval;
use crate::math::{Normal3, Point3, ray::Ray};

pub struct HitRecord {
    /// point where intersection happend
    pub point: Point3,
    /// normal at intersection point
    pub normal: Normal3,
    /// color at intersection point
    pub color: Color,
    /// time of intersection
    pub time: f64,
    /// if the object front facing or back facing
    pub is_front_face: bool,
}

pub trait Intersectable {
    fn intersect(ray: &Ray, interval: Interval) -> Option<HitRecord>;
}
