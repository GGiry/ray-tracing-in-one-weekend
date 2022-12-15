use crate::vec3::Vec3;

use Vec3 as Point3;

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }
}

impl Eq for Ray {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_accessors() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction, 0.0);

        assert_eq!(1.0, ray.origin().x());
        assert_eq!(2.0, ray.origin().y());
        assert_eq!(3.0, ray.origin().z());

        assert_eq!(4.0, ray.direction().x());
        assert_eq!(5.0, ray.direction().y());
        assert_eq!(6.0, ray.direction().z());
    }

    #[test]
    fn ray_at() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);

        let expected_at1 = Point3::new(1.0 + 4.0, 2.0 + 5.0, 3.0 + 6.0);
        let expected_at2 = Point3::new(1.0 + 8.0, 2.0 + 10.0, 3.0 + 12.0);

        let ray = Ray::new(origin, direction, 0.0);

        let at0 = ray.at(0.0);
        let at1 = ray.at(1.0);
        let at2 = ray.at(2.0);

        assert_eq!(origin, at0);
        assert_eq!(expected_at1, at1);
        assert_eq!(expected_at2, at2);
    }
}
