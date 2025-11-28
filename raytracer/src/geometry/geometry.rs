use crate::geometry::sphere::Sphere;

pub enum Geometry<'a> {
    Sphere(Sphere<'a>),
}

impl<'a> Geometry<'a> {}
