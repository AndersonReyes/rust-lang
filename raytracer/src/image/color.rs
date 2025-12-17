use std::ops::{Add, Mul};

pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

/// represents color where values range from 0.0 to 1.0
pub type ColorNormalized = Color<f64>;
/// represents color where values range from 0 to 255
pub type ColorU8 = Color<u8>;
pub type Colorf64 = Color<f64>;

impl<T> Color<T> {
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }

    pub fn as_u8(&self) -> Color<u8>
    where
        T: Into<f64> + Clone,
    {
        Color::new(
            (255.999 * self.r.clone().into()).floor() as u8,
            (255.999 * self.g.clone().into()).floor() as u8,
            (255.999 * self.b.clone().into()).floor() as u8,
        )
    }
}

impl<T: Add<f64, Output = T>> Add<f64> for Color<T> {
    type Output = Color<T>;

    fn add(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.r + rhs, self.g + rhs, self.b + rhs)
    }
}

impl<T: Add<T, Output = T>> Add<Color<T>> for Color<T> {
    type Output = Color<T>;

    fn add(self, rhs: Color<T>) -> Self::Output {
        Self::Output::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

//impl<T: Add<f64, Output = T>> Add<Color<T>> for f64 {
//    type Output = Color<T>;
//
//    fn add(self, rhs: Color<T>) -> Self::Output {
//        rhs.add(self)
//    }
//}

impl<T: Mul<f64, Output = T>> Mul<f64> for Color<T> {
    type Output = Color<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

//impl<T: Mul<f64, Output = T>> Mul<Color<T>> for f64 {
//    type Output = Color<T>;
//    fn mul(self, rhs: Color<T>) -> Self::Output {
//        rhs.mul(self)
//    }
//}

pub const RED: ColorNormalized = Color::new(1.0, 0.0, 0.0);
pub const BLUE: ColorNormalized = Color::new(0.0, 0.0, 1.0);
pub const BLACK: ColorNormalized = Color::new(0.0, 0.0, 0.0);
pub const GREEN: ColorNormalized = Color::new(0.0, 1.0, 0.0);
