#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q_overflow() {
        // 0xc000000000000001, 0x04c0000000000004
        let _ = remquo(-2.0000000000000004, 8.406091369059082e-286);
    }

    /* FOLLOWING TESTS GENERATED BY CHATGPT */
    #[test]
    fn test_remquo_positive() {
        let (remainder, quotient) = remquo(10.0, 3.0);
        assert!((remainder - 1.0).abs() < 1e-6, "Remainder of 10.0 / 3.0 should be 1.0");
        assert_eq!(quotient, 3, "Quotient of 10.0 / 3.0 should be 3");
    }

    #[test]
    fn test_remquo_negative_dividend() {
        let (remainder, quotient) = remquo(-10.0, 3.0);
        assert!((remainder + 1.0).abs() < 1e-6, "Remainder of -10.0 / 3.0 should be -1.0");
        assert_eq!(quotient, -3, "Quotient of -10.0 / 3.0 should be -3");
    }

    #[test]
    fn test_remquo_negative_divisor() {
        let (remainder, quotient) = remquo(10.0, -3.0);
        assert!((remainder - 1.0).abs() < 1e-6, "Remainder of 10.0 / -3.0 should be 1.0");
        assert_eq!(quotient, -3, "Quotient of 10.0 / -3.0 should be -3");
    }

    #[test]
    fn test_remquo_negative_both() {
        let (remainder, quotient) = remquo(-10.0, -3.0);
        assert!((remainder + 1.0).abs() < 1e-6, "Remainder of -10.0 / -3.0 should be -1.0");
        assert_eq!(quotient, 3, "Quotient of -10.0 / -3.0 should be 3");
    }

    #[test]
    fn test_remquo_zero_dividend() {
        let (remainder, quotient) = remquo(0.0, 3.0);
        assert_eq!(remainder, 0.0, "Remainder of 0.0 / 3.0 should be 0.0");
        assert_eq!(quotient, 0, "Quotient of 0.0 / 3.0 should be 0");
    }
}
