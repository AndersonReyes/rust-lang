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

    pub fn random_unit_vector() -> Vector3f {
        loop {
            let v: Vector3f = Vector3f::new(
                rand::random_range(-1.0..=1.0),
                rand::random_range(-1.0..=1.0),
                rand::random_range(-1.0..=1.0),
            );

            let len_squared = v.norm_squared();

            if 1e-160 < len_squared && len_squared <= 1.0 {
                return v.normalize();
            }
        }
    }
}

impl Add for Vector3f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Add<f64> for Vector3f {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Vector3f::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Mul for Vector3f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
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

impl Sub for Vector3f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Div for Vector3f {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f64> for Vector3f {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_vectors() {
        let a = Vector3f::new(1.0, 2.0, -3.0);
        assert_eq!(a + a, Vector3f::new(2., 4., -6.0));
    }

    #[test]
    fn norm_squared() {
        let a = Vector3f::new(1.0, 2.0, -3.0);
        assert_eq!(a.norm_squared(), 14.0);
    }

    #[test]
    fn norm() {
        let a = Vector3f::new(1.0, 2.0, -3.0);
        assert_eq!(a.norm(), 14.0_f64.sqrt());
    }

    #[test]
    fn dot() {
        let a = Vector3f::new(1.0, 2.0, -3.0);
        let b = Vector3f::new(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), -4.0);
    }

    #[test]
    fn normalize() {
        let a = Vector3f::new(1.0, 2.0, -3.0);
        let denom = 14.0_f64.sqrt();
        assert_eq!(
            a.normalize(),
            Vector3f::new(a.x / denom, a.y / denom, a.z / denom)
        );
    }

    #[test]
    fn equals() {
        let a = Vector3f::new(1.0, 2.0, -3.0);
        assert_eq!(a, a);
        assert_eq!(a == a, true);
        assert_eq!(a.eq(&a), true);
    }

    #[test]
    fn random_unit_vector() {
        let a = Vector3f::random_unit_vector();
        assert_eq!(utils::is_close_to(&a.norm(), &1.0), true);
    }
}
