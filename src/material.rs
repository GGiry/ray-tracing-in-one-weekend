use crate::hittable::HitRecord;
use crate::{Color, Ray};

pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}
