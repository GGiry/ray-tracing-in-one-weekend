use crate::hittable::HitRecord;
use crate::material::Material;
use crate::{Color, Ray, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: &Color) -> Lambertian {
        Lambertian { albedo: *color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let result = Ray::new(hit_record.point, scatter_direction, ray.time());
        Some((result, self.albedo))
    }
}

impl PartialEq for Lambertian {
    fn eq(&self, other: &Self) -> bool {
        self.albedo == other.albedo
    }
}

impl Eq for Lambertian {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let color = Color::new(1.0, 0.5, 0.1);
        let lambertian = Lambertian::new(&color);
        assert_eq!(color, lambertian.albedo);
    }

    #[test]
    fn test_eq() {
        let color_a = Color::new(1.0, 0.5, 0.1);
        let lambertian_a = Lambertian::new(&color_a);
        let color_b = Color::new(1.0, 0.5, 0.1);
        let lambertian_b = Lambertian::new(&color_b);
        let color_c = Color::new(1.1, 0.5, 0.1);
        let lambertian_c = Lambertian::new(&color_c);

        assert_eq!(lambertian_a, lambertian_b);
        assert_ne!(lambertian_a, lambertian_c);
    }
}
