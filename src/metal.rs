use crate::hittable::HitRecord;
use crate::material::Material;
use crate::{Color, Ray};

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(color: &Color) -> Metal {
        return Metal { albedo: *color };
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().unit_vector().reflect(&hit_record.normal);
        let result = Ray::new(hit_record.point, reflected);
        return Some((result, self.albedo));
    }
}
