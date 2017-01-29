pub fn gba_to_display_format(gba_format: u16) -> u16 {
    (gba_format & 0b0_00000_00000_11111) << 11 | // Red
        (gba_format & 0b0_00000_11111_00000) << 1  | // Green
        (gba_format & 0b0_11111_00000_00000) >> 9 // Blue
}

#[cfg(test)]
mod tests {
    use super::gba_to_display_format;

    #[test]
    fn test_display_format_conversion() {
        let gba_format = 0b0_10001_11111_00000;
        let display_format = 0b00000_11111_10001_0;

        assert_eq!(gba_to_display_format(gba_format), display_format);
    }
}
