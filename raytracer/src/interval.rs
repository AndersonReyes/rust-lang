#[derive(Clone, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// is x in [min, max]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// is x in (min, max)
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// clamp x in the range of this interval
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
