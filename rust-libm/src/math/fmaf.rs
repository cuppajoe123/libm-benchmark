#[cfg(test)]
mod tests {
    #[test]
    fn issue_263() {
        let a = f32::from_bits(1266679807);
        let b = f32::from_bits(1300234242);
        let c = f32::from_bits(1115553792);
        let expected = f32::from_bits(1501560833);
        assert_eq!(super::fmaf(a, b, c), expected);
    }
}
