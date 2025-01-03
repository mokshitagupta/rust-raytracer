use crate::{INFINTY, NEG_INFINTY};

#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
    // pub empty: Cow<'static, Interval>,
}

impl Interval {
    pub const fn new() -> Interval {
        return Interval {
            min: INFINTY,
            max: NEG_INFINTY,
        };
    }

    pub const fn from(min: f64, max: f64) -> Interval {
        return Interval { min, max };
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if self.min > x {
            return self.min;
        }
        if self.max < x {
            return self.max;
        }
        return x;
    }
}

pub const EMPTY: Interval = Interval::from(INFINTY, -INFINTY);
pub const UNIVERSE: Interval = Interval::from(-INFINTY, INFINTY);
