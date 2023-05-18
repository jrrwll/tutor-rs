use super::alpha_numeric::encode_alpha_numeric;
use super::numeric::encode_numeric;

pub struct QrCode {
    pub version:      u8,
    pub size:         u8,
    // code width
    pub numeric:      u8,
    // 0001
    pub alphanumeric: u8,
    // 0010
    pub bit8:         u8,
    // 0100
    pub kanji:        u8, // 1000
}

impl QrCode {
    fn new(version: u8, size: u8, numeric: u8, alphanumeric: u8, bit8: u8, kanji: u8) -> Self {
        QrCode {
            version,
            size,
            numeric,
            alphanumeric,
            bit8,
            kanji,
        }
    }

    pub fn from_version(version: u8) -> Self {
        let size = ((version - 1) << 2) + 21;
        if version <= 9 {
            Self::new(version, size, 10, 9, 8, 8) // 1-9
        } else if version <= 26 {
            Self::new(version, size, 12, 11, 16, 10) // 10-26
        } else {
            Self::new(version, size, 14, 13, 16, 12) // 27-40
        }
    }

    pub fn encode_numeric(&self, nums: &str) -> Result<Vec<u8>, String> {
        encode_numeric(self.numeric, nums)
    }

    pub fn encode_alpha_numeric(&self, nums: &str) -> Result<Vec<u8>, String> {
        encode_alpha_numeric(self.alphanumeric, nums)
    }
}
