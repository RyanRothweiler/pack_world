use std::ops::{Add, Sub};

pub const fn ms_to_seconds(ms: f64) -> f64 {
    ms / 1000.0
}

pub const fn minutes_to_seconds(minute: f64) -> f64 {
    minute * 60.0
}

pub const fn hours_to_seconds(hour: f64) -> f64 {
    minutes_to_seconds(hour * 60.0)
}

pub const fn days_to_seconds(days: f64) -> f64 {
    hours_to_seconds(days * 24.0)
}

/// This stores as i64 for easy comparisons
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TimeUnit {
    MilliSeconds(i64),
    Minutes(i64),
    Seconds(i64),
    Hours(i64),
    Days(i64),
}

/// Stores time as f64 for accuracy
#[derive(Debug)]
pub struct Time {
    ms: f64,
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
        TimeUnit::MilliSeconds(self.ms as i64)
    }

    pub const fn as_seconds(&self) -> TimeUnit {
        TimeUnit::Seconds((self.ms / 1000.0) as i64)
    }

    pub const fn as_hours(&self) -> TimeUnit {
        TimeUnit::Hours((self.ms / 1000.0 / 60.0) as i64)
    }

    pub const fn as_days(&self) -> TimeUnit {
        TimeUnit::Days((self.ms / 1000.0 / 60.0 / 24.0) as i64)
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
    pub fn conversions() {
        assert_eq!(minutes_to_seconds(1.0), 60.0);
        assert_eq!(hours_to_seconds(1.0), 3600.0);
        assert_eq!(days_to_seconds(1.5), 129600.0);
    }

    #[test]
    pub fn time_build() {
        assert_eq!(
            Time::new(TimeUnit::Seconds(60)),
            Time::new(TimeUnit::Minutes(1))
        );

        assert_eq!(
            Time::new(TimeUnit::Hours(1)),
            Time::new(TimeUnit::Minutes(60))
        );

        assert_eq!(
            Time::new(TimeUnit::Days(1)),
            Time::new(TimeUnit::Minutes(1440))
        );
    }

    #[test]
    pub fn sub() {
        assert_eq!(
            Time::new(TimeUnit::Seconds(60)) - Time::new(TimeUnit::Minutes(1)),
            Time::new(TimeUnit::Seconds(0))
        );
    }

    #[test]
    pub fn add() {
        assert_eq!(
            Time::new(TimeUnit::Minutes(59)) + Time::new(TimeUnit::Minutes(1)),
            Time::new(TimeUnit::Hours(1))
        );
    }
}
