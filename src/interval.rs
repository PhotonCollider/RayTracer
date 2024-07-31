#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new() -> Self {
        Interval {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn with_bounds(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn with_orderless_bounds(a: f64, b: f64) -> Self {
        Interval {
            min: a.min(b),
            max: a.max(b),
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: -f64::INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: -f64::INFINITY,
        max: f64::INFINITY,
    };

    pub fn expand(&self, delta: f64) -> Self {
        Self::with_bounds(self.min - delta / 2.0, self.max + delta / 2.0)
    }

    pub fn intersect(&self, rhs: Self) -> Self {
        Self {
            min: self.min.max(rhs.min),
            max: self.max.min(rhs.max),
        }
    }

    pub fn union(&self, rhs: Self) -> Self {
        Self {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max),
        }
    }

}

impl std::ops::Add<f64> for Interval {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}