pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

/// represents color where values range from 0.0 to 1.0
pub type ColorNormalized = Color<f64>;
/// represents color where values range from 0 to 255
pub type ColorU8 = Color<u8>;

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

pub const RED: ColorNormalized = Color::new(1.0, 0.0, 0.0);
pub const BLUE: ColorNormalized = Color::new(0.0, 0.0, 1.0);
pub const BLACK: ColorNormalized = Color::new(0.0, 0.0, 0.0);
pub const GREEN: ColorNormalized = Color::new(0.0, 1.0, 0.0);
