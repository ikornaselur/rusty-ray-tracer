use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(self) -> Self {
        self.clone() / self.length()
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, t: f32) -> Self::Output {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Self::Output {
        Self {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 2.0);
        assert_eq!(a.z, 3.0);
    }

    #[test]
    fn test_eq() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_add() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 10.0,
            y: 20.0,
            z: 30.0,
        };
        let expected = Vec3 {
            x: 11.0,
            y: 22.0,
            z: 33.0,
        };
        assert_eq!(a + b, expected);
    }
    #[test]
    fn test_sub() {
        let a = Vec3 {
            x: 10.0,
            y: 20.0,
            z: 30.0,
        };
        let b = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let expected = Vec3 {
            x: 9.0,
            y: 18.0,
            z: 27.0,
        };
        assert_eq!(a - b, expected);
    }

    #[test]
    fn test_mul_scalar() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let mul: f32 = 3.0;
        let expected = Vec3 {
            x: 3.0,
            y: 6.0,
            z: 9.0,
        };
        assert_eq!(a * mul, expected);
    }

    #[test]
    fn test_mul_vec3() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let expected = Vec3 {
            x: 2.0,
            y: 6.0,
            z: 12.0,
        };
        assert_eq!(a * b, expected);
    }

    #[test]
    fn test_div() {
        let a = Vec3 {
            x: 11.0,
            y: 22.0,
            z: 33.0,
        };
        let div: f32 = 11.0;
        let expected = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(a / div, expected);
    }

    #[test]
    fn test_div_fraction() {
        let a = Vec3 {
            x: 9.0,
            y: 4.5,
            z: 3.0,
        };
        let div: f32 = 3.0;
        let expected = Vec3 {
            x: 3.0,
            y: 1.5,
            z: 1.0,
        };
        assert_eq!(a / div, expected);
    }

    #[test]
    fn test_dot() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 5.0,
        };
        assert_eq!(a.dot(b), 29.0)
    }

    #[test]
    fn test_cross() {
        let a = Vec3 {
            x: 1.0,
            y: -2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: -4.0,
            y: 5.0,
            z: -5.0,
        };
        let expected = Vec3 {
            x: -5.0,
            y: -7.0,
            z: -3.0,
        };
        assert_eq!(a.cross(b), expected);
    }

    #[test]
    fn test_length_squared() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(a.length_squared(), 14.0);
    }

    #[test]
    fn test_length() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(a.length(), 14.0_f32.sqrt());
    }

    #[test]
    fn test_fmt() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(format!("{}", a), "1 2 3");
    }

    #[test]
    fn test_unit_vector() {
        let a = Vec3 {
            x: 2.0,
            y: 4.0,
            z: 4.0,
        };
        let expected = Vec3 {
            x: 2.0 / 6.0,
            y: 4.0 / 6.0,
            z: 4.0 / 6.0,
        };
        assert_eq!(a.unit_vector(), expected);
    }
}
