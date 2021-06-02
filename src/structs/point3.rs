use std::fmt;
use std::ops::{Add, Sub};

use crate::structs::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
impl Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for Point3 {
    type Output = Vec3;

    fn sub(self, other: Point3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vec3_to_point3() {
        let point = Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let expected = Point3 {
            x: 5.0,
            y: 7.0,
            z: 9.0,
        };
        assert_eq!(point + vec, expected);
    }

    #[test]
    fn test_point3_sub_vec3_to_point3() {
        let point = Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec = Vec3 {
            x: 4.1,
            y: 5.2,
            z: 6.3,
        };
        let expected = Point3 {
            x: -3.1,
            y: -3.1999998,
            z: -3.3000002,
        };
        assert_eq!(point - vec, expected);
    }

    #[test]
    fn test_point3_sub_point3_to_vec3() {
        let a = Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Point3 {
            x: 0.1,
            y: 0.2,
            z: 0.3,
        };
        let expected = Vec3 {
            x: 0.9,
            y: 1.8,
            z: 2.7,
        };
        assert_eq!(a - b, expected);
    }
}
