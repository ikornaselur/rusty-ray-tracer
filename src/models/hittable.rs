use crate::structs::{Point3, Ray, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub time: f32,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
