use crate::vec3::Vec3;

use Vec3 as Point3;

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        return Ray { origin, direction };
    }

    pub fn origin(&self) -> Point3 {
        return self.origin;
    }

    pub fn direction(&self) -> Vec3 {
        return self.direction;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_default() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(1.0, ray.origin().x());
        assert_eq!(2.0, ray.origin().y());
        assert_eq!(3.0, ray.origin().z());

        assert_eq!(4.0, ray.direction().x());
        assert_eq!(5.0, ray.direction().y());
        assert_eq!(6.0, ray.direction().z());
    }
}
