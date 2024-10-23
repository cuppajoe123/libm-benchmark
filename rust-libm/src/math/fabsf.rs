// PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
#[cfg(not(target_arch = "powerpc64"))]
#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::*;

    #[test]
    fn sanity_check() {
        assert_eq!(fabsf(-1.0), 1.0);
        assert_eq!(fabsf(2.8), 2.8);
    }

    /// The spec: https://en.cppreference.com/w/cpp/numeric/math/fabs
    #[test]
    fn spec_tests() {
        assert!(fabsf(NAN).is_nan());
        for f in [0.0, -0.0].iter().copied() {
            assert_eq!(fabsf(f), 0.0);
        }
        for f in [INFINITY, NEG_INFINITY].iter().copied() {
            assert_eq!(fabsf(f), INFINITY);
        }
    }
}
