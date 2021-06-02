mod structs;

use std::env;

use indicatif::ProgressBar;
use std::fs::File;
use std::io::prelude::*;
use structs::{Pixel, Point3, Ray, Vec3};

fn main() -> std::io::Result<()> {
    // Image
    let args: Vec<String> = env::args().collect();
    let image_width: usize = args[1].parse().expect("Missing image width argument");

    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    println!("Creating {}x{} image", image_width, image_height);

    // Camera
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

    // Render
    let mut file = File::create("out.ppm")?;
    let mut buf = String::new();
    buf.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    let bar = ProgressBar::new(image_height as u64);

    for row in 0..image_height {
        for column in 0..image_width {
            let u = column as f32 / (image_width - 1) as f32;
            let v = (image_height - row) as f32 / (image_height - 1) as f32;
            let ray = Ray {
                origin: origin,
                direction: lower_left_corner.clone()
                    + u * horizontal.clone()
                    + v * vertical.clone()
                    - origin,
            };
            let pixel = ray_colour(ray);
            buf.push_str(&format!("{} ", pixel));
        }
        bar.inc(1);
        buf.push_str("\n");
    }

    file.write_all(buf.as_bytes())?;

    Ok(())
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

fn ray_colour(ray: Ray) -> Pixel {
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
            * Pixel {
                r: normal.x + 1.0,
                g: normal.y + 1.0,
                b: normal.z + 1.0,
            };
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * unit_direction.y + 1.0;
    (1.0 - t)
        * Pixel {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
        + t * Pixel {
            r: 0.5,
            g: 0.7,
            b: 1.0,
        }
}
