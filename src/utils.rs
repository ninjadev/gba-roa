#[derive(Debug)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    pub fn as_u8(&self) -> (u8, u8, u8){
        (0, 0, 0)
    }
}

impl From<u16> for RGB {
    fn from(num: u16) -> Self {
        RGB {
            red:   (num & 0b0000000000011111) as u8,
            green: ((num & 0b0000001111100000) >> 5) as u8,
            blue:  ((num & 0b0111110000000000) >> 10) as u8,
        }
    }
}
