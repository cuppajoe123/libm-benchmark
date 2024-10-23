#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::*;

    #[test]
    fn sanity_check() {
        assert_eq!(fabs(-1.0), 1.0);
        assert_eq!(fabs(2.8), 2.8);
    }

    /// The spec: https://en.cppreference.com/w/cpp/numeric/math/fabs
    #[test]
    fn spec_tests() {
        assert!(fabs(NAN).is_nan());
        for f in [0.0, -0.0].iter().copied() {
            assert_eq!(fabs(f), 0.0);
        }
        for f in [INFINITY, NEG_INFINITY].iter().copied() {
            assert_eq!(fabs(f), INFINITY);
        }
    }
}
