// PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
#[cfg(not(target_arch = "powerpc64"))]
#[cfg(test)]
mod tests {
    use super::rint;

    #[test]
    fn negative_zero() {
        assert_eq!(rint(-0.0_f64).to_bits(), (-0.0_f64).to_bits());
    }

    #[test]
    fn sanity_check() {
        assert_eq!(rint(-1.0), -1.0);
        assert_eq!(rint(2.8), 3.0);
        assert_eq!(rint(-0.5), -0.0);
        assert_eq!(rint(0.5), 0.0);
        assert_eq!(rint(-1.5), -2.0);
        assert_eq!(rint(1.5), 2.0);
    }
}
