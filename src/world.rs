use crate::models::{HitRecord, Hittable};
use crate::structs::{Colour, Point3, Ray, Vec3};

const MOVE_SPEED: f32 = 0.1;

pub struct World {
    height: u32,
    width: u32,
    focal_length: f32,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    pub hittable_list: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new(height: u32, width: u32) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let horizontal = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vec3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let lower_left_corner: Point3 = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };
        Self {
            height,
            width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            hittable_list: Vec::new(),
        }
    }

    pub fn update(&mut self, right: bool, left: bool, up: bool, down: bool) -> bool {
        // TODO: Proper input handling, this works for just hacking around with now
        if right {
            self.origin.x += MOVE_SPEED;
        } else if left {
            self.origin.x -= MOVE_SPEED;
        }
        if up {
            self.origin.y += MOVE_SPEED;
        } else if down {
            self.origin.y -= MOVE_SPEED;
        }

        if right || left || up || down {
            self.lower_left_corner = self.origin
                - self.horizontal / 2.0
                - self.vertical / 2.0
                - Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: self.focal_length,
                };
            true
        } else {
            false
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let u = (i % self.width as usize) as f32 / (self.width - 1) as f32;
            let v =
                (self.height - (i / self.width as usize) as u32) as f32 / (self.height - 1) as f32;
            let ray = Ray {
                origin: self.origin,
                direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                    - Point3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
            };
            let colour = self.ray_colour(ray);
            pixel.copy_from_slice(&colour.slice_u8());
        }
    }

    fn hit_object(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut closest_hit_record: Option<HitRecord> = None;

        for object in self.hittable_list.iter() {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.time;
                closest_hit_record = Some(hit_record);
            }
        }
        closest_hit_record
    }

    fn ray_colour(&self, ray: Ray) -> Colour {
        if let Some(hit_record) = self.hit_object(ray, 0.0, f32::INFINITY) {
            return 0.5
                * Colour {
                    r: hit_record.normal.x + 1.0,
                    g: hit_record.normal.y + 1.0,
                    b: hit_record.normal.z + 1.0,
                };
        }
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * unit_direction.y + 1.0;
        (1.0 - t)
            * Colour {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            }
            + t * Colour {
                r: 0.5,
                g: 0.7,
                b: 1.0,
            }
    }
}
