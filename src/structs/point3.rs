use std::fmt;
use std::ops::Add;

use crate::structs::Vec3;

#[derive(Debug, PartialEq)]
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
}
