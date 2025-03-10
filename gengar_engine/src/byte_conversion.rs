pub const fn bytes_to_kilobytes(bytes: i64) -> f64 {
    bytes as f64 / 1024.0
}

pub const fn kilobyte_to_bytes(kb: f64) -> i64 {
    (kb * 1024.0) as i64
}

pub const fn bytes_to_megabytes(bytes: i64) -> f64 {
    bytes_to_kilobytes(bytes) / 1024.0
}

pub const fn megabytes_to_bytes(mb: f64) -> i64 {
    kilobyte_to_bytes(mb) * 1024
}

mod test {
    use super::*;

    #[test]
    fn conversions() {
        assert_eq!(kilobyte_to_bytes(1.0), 1024);
        assert_eq!(bytes_to_kilobytes(1024), 1.0);

        assert_eq!(megabytes_to_bytes(1.0), 1_048_576);
        assert_eq!(bytes_to_megabytes(1024 * 1024), 1.0);
    }
}
