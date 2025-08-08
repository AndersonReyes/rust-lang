use std::ops::Add;

use crate::nalgebra::Vector3;

pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    /// Get the ray location at time t
    pub fn at(self: &Self, t: f64) -> Vector3<f64> {
        self.origin.add(self.direction.scale(t))
    }

    pub fn get_direction(self: &Self) -> Vector3<f64> {
        self.direction.clone()
    }

    pub fn get_origin(self: &Self) -> Vector3<f64> {
        self.origin.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ray_at() {
        let ray = Ray::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(1.0, 0.0, 0.0));

        let actual = ray.at(2.0);

        assert_eq!(actual, Vector3::new(3.0, 2.0, 3.0))
    }
}
