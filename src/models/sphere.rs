use crate::models::{HitRecord, Hittable};
use crate::structs::{Point3, Ray};

pub struct Sphere {
    pub radius: f32,
    pub center: Point3,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);

        let mut hit_record = HitRecord {
            point: point,
            normal: (point - self.center) / self.radius,
            time: root,
            front_face: false,
        };
        hit_record.set_face_normal(ray, hit_record.normal);

        Some(hit_record)
    }
}
