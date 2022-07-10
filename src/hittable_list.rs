use crate::hittable::{HitRecord, Hittable};
use crate::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        return HittableList { objects: vec![] };
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                hit_anything = Some(hit);
                closest_so_far = hit.t;
            }
        }

        return hit_anything;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;
    use crate::Point3;

    #[test]
    fn test_init() {
        let hittables = HittableList::new();

        assert_eq!(0, hittables.objects.len());
    }

    #[test]
    fn test_add() {
        let mut hittables = HittableList::new();
        let sphere = Box::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 3.14));
        hittables.add(sphere);

        assert_eq!(1, hittables.objects.len());
    }

    #[test]
    fn test_clear() {
        let mut hittables = HittableList::new();
        let sphere = Box::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 3.14));
        hittables.add(sphere);
        hittables.clear();

        assert_eq!(0, hittables.objects.len());
    }
}
