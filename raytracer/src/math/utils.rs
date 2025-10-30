use nalgebra::Vector3;

use crate::math::Vector3f;
use rand::prelude::*;

pub fn random_unit_vector() -> Vector3f {
    let mut rng = rand::rng();
    loop {
        let v: Vector3f = Vector3::new(
            rng.random_range(-1.0..=1.0),
            rand::random_range(-1.0..=1.0),
            rand::random_range(-1.0..=1.0),
        );

        let len_squared = v.norm_squared();
        if 1e-160 < len_squared && len_squared <= 1.0 {
            return v.normalize();
        }
    }
}

const NEAR_ZERO: f64 = 1e-8;

pub fn near_zero_vector(v: &Vector3f) -> bool {
    v.x.abs() < NEAR_ZERO && v.y.abs() < NEAR_ZERO && v.z.abs() < NEAR_ZERO
}
