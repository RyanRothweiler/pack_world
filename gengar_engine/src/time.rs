pub const fn minutes_to_seconds(minute: f64) -> f64 {
    minute * 60.0
}

pub const fn hours_to_seconds(hour: f64) -> f64 {
    minutes_to_seconds(hour * 60.0)
}

pub const fn days_to_seconds(days: f64) -> f64 {
    hours_to_seconds(days * 24.0)
}

mod test {
    use super::*;

    #[test]
    pub fn conversions() {
        assert_eq!(minutes_to_seconds(1.0), 60.0);
        assert_eq!(hours_to_seconds(1.0), 3600.0);
        assert_eq!(days_to_seconds(1.5), 129600.0);
    }
}
