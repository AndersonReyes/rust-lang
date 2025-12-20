use std::ops::Add;

use crate::math::Vector3f;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3f,
    pub direction: Vector3f,
}

impl Ray {
    pub fn new(origin: Vector3f, direction: Vector3f) -> Self {
        Self { origin, direction }
    }

    /// Get the ray location at time t
    pub fn at(self: &Self, t: f64) -> Vector3f {
        self.origin.add(self.direction * t)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ray_at() {
        let ray = Ray::new(Vector3f::new(1.0, 2.0, 3.0), Vector3f::new(1.0, 0.0, 0.0));

        let actual = ray.at(2.0);

        assert_eq!(actual, Vector3f::new(3.0, 2.0, 3.0))
    }
}
