use crate::hittable::HitRecord;
use crate::material::Material;
use crate::{Color, Ray, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(color: &Color, fuzziness: f64) -> Metal {
        Metal {
            albedo: *color,
            fuzziness,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().unit_vector().reflect(&hit_record.normal);
        let result = Ray::new(
            hit_record.point,
            reflected + self.fuzziness * Vec3::random_in_unit_sphere(),
            ray.time(),
        );
        Some((result, self.albedo))
    }
}
