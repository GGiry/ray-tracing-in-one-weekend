use crate::hittable::{HitRecord, Hittable};
use crate::{dot, Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        return Sphere { center, radius };
    }
}

impl Hittable for Sphere {
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

        let mut hit = HitRecord::default();
        hit.t = root;
        hit.point = ray.at(root);
        let outward_normal = (hit.point - self.center) / self.radius;
        hit.set_face_normal(ray, &outward_normal);

        return Some(hit);
    }
}

impl PartialEq<Self> for Sphere {
    fn eq(&self, other: &Self) -> bool {
        return self.center == other.center && self.radius == other.radius;
    }
}

impl Eq for Sphere {}