use crate::structs::Point3;
use crate::structs::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn at(self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_at() {
        let ray = Ray {
            origin: Point3 {
                x: 1.0,
                y: 1.1,
                z: 1.2,
            },
            direction: Vec3 {
                x: 1.0,
                y: -2.0,
                z: 0.0,
            },
        };

        let expected = Point3 {
            x: 6.0,
            y: -8.9,
            z: 1.2,
        };

        assert_eq!(ray.at(5.0), expected);
    }
}
