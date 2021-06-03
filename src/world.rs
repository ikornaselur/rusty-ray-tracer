use crate::structs::{Colour, Point3, Ray, Vec3};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const MOVE_SPEED: f32 = 0.1;

pub struct World {
    focal_length: f32,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl World {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
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
            focal_length: focal_length,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }

    pub fn update(&mut self, right: bool, left: bool, up: bool, down: bool) {
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
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let u = (i % WIDTH as usize) as f32 / (WIDTH - 1) as f32;
            let v = (HEIGHT - (i / WIDTH as usize) as u32) as f32 / (HEIGHT - 1) as f32;
            let ray = Ray {
                origin: self.origin,
                direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                    - self.origin,
            };
            let colour = ray_colour(ray);
            pixel.copy_from_slice(&colour.slice_u8());
        }
    }
}

fn hit_sphere(center: Point3, radius: f32, ray: Ray) -> f32 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    return if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    };
}

fn ray_colour(ray: Ray) -> Colour {
    let sphere = Point3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let t = hit_sphere(sphere, 0.5, ray);
    if t > 0.0 {
        let normal = (ray.at(t)
            - Point3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            })
        .unit_vector();
        return 0.5
            * Colour {
                r: normal.x + 1.0,
                g: normal.y + 1.0,
                b: normal.z + 1.0,
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
