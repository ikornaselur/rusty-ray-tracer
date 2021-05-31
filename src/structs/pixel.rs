use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn from_f32(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: (u8::MAX as f32 * r) as u8,
            g: (u8::MAX as f32 * g) as u8,
            b: (u8::MAX as f32 * b) as u8,
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_f32() {
        let actual = Pixel::from_f32(0.0, 0.5, 1.0);
        let expected = Pixel {
            r: 0,
            g: 127,
            b: 255,
        };

        assert_eq!(actual, expected);
    }
}
