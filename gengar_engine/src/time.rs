use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub enum TimeUnit {
    MilliSeconds(f64),
    Seconds(f64),
    Minutes(f64),
    Hours(f64),
    Days(f64),
}

impl TimeUnit {
    pub const fn value(&self) -> f64 {
        match self {
            Self::Days(d)
            | Self::Hours(d)
            | Self::Minutes(d)
            | Self::Seconds(d)
            | Self::MilliSeconds(d) => *d,
        }
    }
}

/// Stores time as f64 for accuracy
#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub ms: f64,
}

impl Time {
    pub const fn new(init: TimeUnit) -> Self {
        match init {
            TimeUnit::MilliSeconds(ms) => Self { ms: ms as f64 },
            TimeUnit::Seconds(s) => Self {
                ms: s as f64 * 1000.0,
            },
            TimeUnit::Minutes(m) => Self {
                ms: m as f64 * 60.0 * 1000.0,
            },
            TimeUnit::Hours(h) => Self {
                ms: h as f64 * 60.0 * 60.0 * 1000.0,
            },
            TimeUnit::Days(d) => Self {
                ms: d as f64 * 24.0 * 60.0 * 60.0 * 1000.0,
            },
        }
    }

    pub const fn as_milliseconds(&self) -> TimeUnit {
        TimeUnit::MilliSeconds(self.ms)
    }

    pub const fn as_seconds(&self) -> TimeUnit {
        TimeUnit::Seconds(self.ms / 1000.0)
    }

    pub const fn as_hours(&self) -> TimeUnit {
        TimeUnit::Hours(self.ms / 1000.0 / 60.0)
    }

    pub const fn as_days(&self) -> TimeUnit {
        TimeUnit::Days(self.ms / 1000.0 / 60.0 / 24.0)
    }

    pub fn clamp_ms(&mut self, min: f64, max: f64) {
        self.ms = self.ms.clamp(min, max);
    }

    pub const fn greater_than_zero(&self) -> bool {
        self.ms > 0.0
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 1e-9;

        (self.ms - other.ms).abs() < EPSILON
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            ms: self.ms - other.ms,
        }
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            ms: self.ms + other.ms,
        }
    }
}

mod test {
    use super::*;

    #[test]
    pub fn time_build() {
        assert_eq!(
            Time::new(TimeUnit::Seconds(60.0)),
            Time::new(TimeUnit::Minutes(1.0))
        );

        assert_eq!(
            Time::new(TimeUnit::Hours(1.0)),
            Time::new(TimeUnit::Minutes(60.0))
        );

        assert_eq!(
            Time::new(TimeUnit::Days(1.0)),
            Time::new(TimeUnit::Minutes(1440.0))
        );
    }

    #[test]
    pub fn sub() {
        assert_eq!(
            Time::new(TimeUnit::Seconds(60.0)) - Time::new(TimeUnit::Minutes(1.0)),
            Time::new(TimeUnit::Seconds(0.0))
        );
    }

    #[test]
    pub fn add() {
        assert_eq!(
            Time::new(TimeUnit::Minutes(59.0)) + Time::new(TimeUnit::Minutes(1.0)),
            Time::new(TimeUnit::Hours(1.0))
        );
    }
}
