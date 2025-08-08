use crate::nalgebra::Vector3;

// TODO: should I just wrap the vector class to hide scale between [0,1] or [0,255]?
/// Color as a unit vector
pub type Color = Vector3<f64>;
