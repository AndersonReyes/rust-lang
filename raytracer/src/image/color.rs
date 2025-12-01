use crate::math::Vector3f;

// TODO: should I just wrap the vector class to hide scale between [0,1] or [0,255]?
/// Color as a unit vector
pub type Color = Vector3f;

pub const RED: Color = Color::new(1.0, 0.0, 0.0);
pub const BLUE: Color = Color::new(0.0, 0.0, 1.0);
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const GREEN: Color = Color::new(0.0, 1.0, 0.0);

pub fn as_u8(v: f64) -> u8 {
    (255.999 * v).floor() as u8
}
