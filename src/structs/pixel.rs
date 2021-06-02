use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Pixel {
    fn r_u8(self) -> u8 {
        (self.r * (u8::MAX as f32)) as u8
    }
    fn g_u8(self) -> u8 {
        (self.g * (u8::MAX as f32)) as u8
    }
    fn b_u8(self) -> u8 {
        (self.b * (u8::MAX as f32)) as u8
    }
}

impl Mul<f32> for Pixel {
    type Output = Self;

    fn mul(self, t: f32) -> Self::Output {
        Self {
            r: self.r * t,
            g: self.g * t,
            b: self.b * t,
        }
    }
}

impl Mul<Pixel> for f32 {
    type Output = Pixel;

    fn mul(self, other: Pixel) -> Self::Output {
        other * self
    }
}

impl Add for Pixel {
    type Output = Self;

    fn add(self, other: Pixel) -> Self::Output {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r_u8(), self.g_u8(), self.b_u8())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_f32() {
        let actual = Pixel {
            r: 0.0,
            g: 0.5,
            b: 1.0,
        };
        assert_eq!(actual.r_u8(), 0);
        assert_eq!(actual.g_u8(), 127);
        assert_eq!(actual.b_u8(), 255);
    }
}
