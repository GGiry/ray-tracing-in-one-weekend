use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::{dot, Point3, Ray, Vec3};

pub struct MovingSphere<Mat: Material> {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Mat,
}

impl<Mat: Material> MovingSphere<Mat> {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Mat,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            radius,
            material,
            time0,
            time1,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<Mat: Material> Hittable for MovingSphere<Mat> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
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

        let outward_normal = (hit.point - self.center(ray.time())) / self.radius;
        hit.set_face_normal(ray, &outward_normal);

        Some(hit)
    }
}

impl<Mat: Material> PartialEq<Self> for MovingSphere<Mat> {
    fn eq(&self, other: &Self) -> bool {
        self.center0 == other.center0
            && self.center1 == other.center1
            && self.radius == other.radius
    }
}

impl<Mat: Material> Eq for MovingSphere<Mat> {}
