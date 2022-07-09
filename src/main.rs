use std::fs::File;
use std::io::Write;

mod ray;
mod utils;
mod vec3;

use crate::utils::write_color;
use crate::vec3::{dot, Vec3};

use crate::ray::Ray;
use Vec3 as Color;
use Vec3 as Point3;

fn linear_blend(t: f64, start: Color, end: Color) -> Color {
    return (1.0 - t) * start + t * end;
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(oc, ray.direction());
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    return (-b - discriminant.sqrt()) / (2.0 * a);
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }
    let unit_direction = ray.direction().unit_vector();
    let white = Color::new(1.0, 1.0, 1.0);
    let light_blue = Color::new(0.5, 0.7, 1.0);
    let t = 0.5 * (unit_direction.y() + 1.0);

    return linear_blend(t, white, light_blue);
}

fn create_gradient(path: &str) {
    let mut file = File::create(path).unwrap();

    let image_width = 256;
    let image_height = 256;

    file.write_all(b"P3\n").expect("Unable to write data");
    file.write_all(format!("{image_width} {image_height}\n").as_bytes())
        .expect("Unable to write data");
    file.write_all(b"255\n").expect("Unable to write data");

    for index_height in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {index_height}");
        for index_width in 0..image_width {
            let pixel = Color::new(
                index_width as f64 / ((image_width - 1) as f64),
                index_height as f64 / ((image_height - 1) as f64),
                0.25,
            );

            write_color(&mut file, pixel);
        }
    }
    eprintln!("Done.");
}

fn main() {
    // Image file
    let mut file = File::create("result.ppm").unwrap();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    file.write_all(b"P3\n").expect("Unable to write data");
    file.write_all(format!("{image_width} {image_height}\n").as_bytes())
        .expect("Unable to write data");
    file.write_all(b"255\n").expect("Unable to write data");

    for index_height in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {index_height}");
        for index_width in 0..image_width {
            let u = (index_width as f64) / ((image_width - 1) as f64);
            let v = (index_height as f64) / ((image_height - 1) as f64);

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&ray);

            write_color(&mut file, pixel_color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_diff::diff;

    #[test]
    fn gradient_ppm() {
        let result_path = "result_gradient.ppm";
        create_gradient(result_path);
        assert_eq!(true, diff("expected_gradient.ppm", result_path));
    }

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

    #[test]
    fn hit_sphere_test() {
        let center = Point3::new(0.0, 0.0, 0.0);
        let radius = 1.0;

        let origin = Point3::new(0.0, 0.0, 3.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray_touching = Ray::new(origin, direction);

        let direction2 = Vec3::new(0.0, 1.0, 0.0);
        let ray_not_touching = Ray::new(origin, direction2);

        assert!(hit_sphere(center, radius, &ray_touching));
        assert!(!hit_sphere(center, radius, &ray_not_touching));
    }
}
