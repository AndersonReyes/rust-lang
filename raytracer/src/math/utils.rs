use crate::math::Vector3f;

pub fn random_double() -> f64 {
    rand::random_range(-1.0..=1.0)
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

const NEAR_ZERO: f64 = 1e-8;

pub fn near_zero_vector(v: &Vector3f) -> bool {
    v.x.abs() < NEAR_ZERO && v.y.abs() < NEAR_ZERO && v.z.abs() < NEAR_ZERO
}

pub fn is_close_to<T: Into<f64> + Copy>(a: &T, b: &T) -> bool {
    let aa: f64 = (*a).into();
    let bb: f64 = (*b).into();
    (aa - bb).abs() < NEAR_ZERO
}
