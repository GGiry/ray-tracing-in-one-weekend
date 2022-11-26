use std::fs::File;
use std::io::Write;

use rayon::prelude::*;

use Vec3 as Color;
use Vec3 as Point3;

use crate::camera::Camera;
use crate::dielectric::Dielectric;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::{color_to_rbg, random_f64, random_f64_range};
use crate::vec3::{dot, Vec3};

mod camera;
mod dielectric;
mod hittable;
mod hittable_list;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn linear_blend(t: f64, start: Color, end: Color) -> Color {
    (1.0 - t) * start + t * end
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    let white = Color::new(1.0, 1.0, 1.0);
    let light_blue = Color::new(0.5, 0.7, 1.0);

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::default();
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    linear_blend(t, white, light_blue)
}

#[allow(dead_code)]
fn scene() -> HittableList {
    let mut world_mut = HittableList::new();

    let ground = Lambertian::new(&Color::new(0.8, 0.8, 0.0));
    let center = Lambertian::new(&Color::new(0.1, 0.2, 0.5));
    let left = Dielectric::new(1.5);
    let right = Metal::new(&Color::new(0.8, 0.6, 0.2), 0.0);

    world_mut.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        ground,
    )));
    world_mut.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        center,
    )));
    world_mut.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        left,
    )));
    world_mut.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        left,
    )));
    world_mut.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        right,
    )));

    world_mut
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(&Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            let random = random_f64();
            if random < 0.8 {
                // diffuse
                let albedo = Color::random() * Color::random();
                world.add(Box::new(Sphere::new(center, 0.2, Lambertian::new(&albedo))));
            } else if random < 0.95 {
                // metal
                world.add(Box::new(Sphere::new(
                    center,
                    0.2,
                    Metal::new(&Color::random(), random_f64_range(0.0, 0.5)),
                )));
            } else {
                // glass
                world.add(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(&Color::new(0.4, 0.2, 0.1)),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0),
    )));

    world
}

fn main() {
    // Image
    let vertical_field_of_view = 20.0;
    let aspect_ratio = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let camera = Camera::new(
        &look_from,
        &look_at,
        &view_up,
        vertical_field_of_view,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    let image = (0..image_height)
        .into_par_iter()
        .rev()
        .flat_map(|index_height| {
            eprintln!("Lines remaining: {index_height}");
            (0..image_width)
                .flat_map(|index_width| {
                    let pixel_color: Vec3 = (0..samples_per_pixel)
                        .map(|_| {
                            let u =
                                (index_width as f64 + random_f64()) / ((image_width - 1) as f64);
                            let v =
                                (index_height as f64 + random_f64()) / ((image_height - 1) as f64);
                            let ray = camera.get_ray(u, v);
                            ray_color(&ray, &world, max_depth)
                        })
                        .sum();

                    color_to_rbg(pixel_color, samples_per_pixel)
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    // Write result to file
    let mut file = File::create("result.ppm").unwrap();
    file.write_all(b"P3\n").expect("Unable to write data");
    file.write_all(format!("{image_width} {image_height}\n").as_bytes())
        .expect("Unable to write data");
    file.write_all(b"255\n").expect("Unable to write data");

    for rgb in image.chunks(3) {
        file.write_all(format!("{} {} {}\n", rgb[0], rgb[1], rgb[2]).as_bytes())
            .expect("Unable to write data");
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
