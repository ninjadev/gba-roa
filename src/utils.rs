pub fn u1u5u5u5_to_u5u5u5u1(u1u5u5u5: u16) -> u16 {
    (u1u5u5u5 & 0b0_00000_00000_11111) << 11 | // Red
        (u1u5u5u5 & 0b0_00000_11111_00000) << 1  | // Green
        (u1u5u5u5 & 0b0_11111_00000_00000) >> 9 // Blue
}

#[cfg(test)]
mod tests {
    use super::u1u5u5u5_to_u5u5u5u1;

    #[test]
    fn test_display_format_conversion() {
        let gba_format = 0b0_10001_11111_00000;
        let display_format = 0b00000_11111_10001_0;

        assert_eq!(u1u5u5u5_to_u5u5u5u1(gba_format), display_format);
    }
}
