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

impl HitRecord {
    pub fn new(point: Point3, normal: Normal3, color: Color, time: f64, ray: &Ray) -> Self {
        let is_front_face = ray.direction.dot(&normal) < 0.0;
        let norm = if is_front_face { normal } else { -normal };

        Self {
            point: point,
            normal: norm,
            color: color,
            time: time,
            is_front_face: is_front_face,
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}
