use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::utils;

#[derive(Debug, Clone, Copy)]
pub struct Vector3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialEq for Vector3f {
    fn eq(&self, other: &Self) -> bool {
        utils::is_close_to(&self.x, &other.x)
            && utils::is_close_to(&self.y, &other.y)
            && utils::is_close_to(&self.z, &other.z)
    }
}

impl Eq for Vector3f {}

impl Vector3f {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let l = self.norm();
        Self::new(self.x / l, self.y / l, self.z / l)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vector3f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f64> for Vector3f {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Neg for Vector3f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_vectors() {
        let a = Vector3f::new(1.0, 2.0, -3.0);
        assert_eq!(a + a, Vector3f::new(2., 4., 0.));
    }
}
