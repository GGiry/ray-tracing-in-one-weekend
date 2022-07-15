use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::{dot, Point3, Ray, Vec3};

pub struct Sphere<Mat: Material> {
    center: Point3,
    radius: f64,
    material: Mat,
}

impl<Mat: Material> Sphere<Mat> {
    pub fn new(center: Point3, radius: f64, material: Mat) -> Self {
        return Sphere {
            center,
            radius,
            material,
        };
    }
}

impl<Mat: Material> Hittable for Sphere<Mat> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(&oc, &ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();

        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut hit = HitRecord {
            point: ray.at(root),
            normal: Vec3::default(),
            t: root,
            front_face: false,
            material: &self.material,
        };

        let outward_normal = (hit.point - self.center) / self.radius;
        hit.set_face_normal(ray, &outward_normal);

        return Some(hit);
    }
}

impl<Mat: Material> PartialEq<Self> for Sphere<Mat> {
    fn eq(&self, other: &Self) -> bool {
        return self.center == other.center && self.radius == other.radius;
    }
}

impl<Mat: Material> Eq for Sphere<Mat> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Lambertian};

    #[test]
    fn new_test() {
        let lambertian_white = Lambertian::new(&Color::new(1.0, 1.0, 1.0));
        let sphere = Sphere::new(Point3::default(), 1.0, lambertian_white);

        assert_eq!(sphere.center, Point3::default());
        assert_eq!(sphere.radius, 1.0);
        assert_eq!(sphere.material, lambertian_white);
    }

    #[test]
    fn test_miss() {
        let lambertian_white = Lambertian::new(&Color::new(1.0, 1.0, 1.0));
        let sphere = Sphere::new(Point3::default(), 1.0, lambertian_white);

        let hit_record = sphere.hit(
            &Ray::new(Point3::new(0.0, 0.0, 2.0), Vec3::new(1.0, 0.0, 0.0)),
            0.1,
            f64::INFINITY,
        );

        assert!(hit_record.is_none());
    }

    #[test]
    fn test_hit_simple() {
        let lambertian_white = Lambertian::new(&Color::new(1.0, 1.0, 1.0));
        let sphere = Sphere::new(Point3::default(), 1.0, lambertian_white);

        match sphere.hit(
            &Ray::new(Point3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0)),
            0.1,
            f64::INFINITY,
        ) {
            None => assert!(false),
            Some(hit_record) => {
                assert_eq!(1.0, hit_record.t); // Sphere is hit in one "step"
                assert_eq!(Vec3::new(0.0, 0.0, 1.0), hit_record.normal); // -direction of the ray
                assert_eq!(Vec3::new(0.0, 0.0, 1.0), hit_record.point); // position of the hit
                assert_eq!(true, hit_record.front_face); // hit from the exterior
            }
        }
    }
}
