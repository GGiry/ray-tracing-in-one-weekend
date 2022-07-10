use rand::Rng;

use std::fs::File;
use std::io::Write;

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::utils::write_color;
use crate::vec3::{dot, Vec3};

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use Vec3 as Color;
use Vec3 as Point3;

fn linear_blend(t: f64, start: Color, end: Color) -> Color {
    return (1.0 - t) * start + t * end;
}

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let white = Color::new(1.0, 1.0, 1.0);
    let light_blue = Color::new(0.5, 0.7, 1.0);

    if let Some(hit) = world.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * (hit.normal + white);
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    return linear_blend(t, white, light_blue);
}

fn main() {
    // Image file
    let mut file = File::create("result.ppm").unwrap();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    let mut rng = rand::thread_rng();

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    file.write_all(b"P3\n").expect("Unable to write data");
    file.write_all(format!("{image_width} {image_height}\n").as_bytes())
        .expect("Unable to write data");
    file.write_all(b"255\n").expect("Unable to write data");

    for index_height in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {index_height}");
        for index_width in 0..image_width {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (index_width as f64 + rng.gen::<f64>()) / ((image_width - 1) as f64);
                let v = (index_height as f64 + rng.gen::<f64>()) / ((image_height - 1) as f64);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }
            write_color(&mut file, pixel_color, samples_per_pixel);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_blend_test() {
        let black = Color::new(0.0, 0.0, 0.0);
        let white = Color::new(1.0, 1.0, 1.0);
        let expected_grey = Color::new(0.5, 0.5, 0.5);

        let grey_0_0 = linear_blend(0.0, white, black);
        let grey_0_5 = linear_blend(0.5, white, black);
        let grey_1_0 = linear_blend(1.0, white, black);

        assert_eq!(white, grey_0_0);
        assert_eq!(expected_grey, grey_0_5);
        assert_eq!(black, grey_1_0);
    }
}
