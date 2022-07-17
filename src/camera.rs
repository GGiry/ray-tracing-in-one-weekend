use crate::vec3::cross;
use crate::{Point3, Ray, Vec3};

#[derive(Clone, Copy, Default)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        view_up: &Vec3,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vertical_field_of_view.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (*look_from - *look_at).unit_vector();
        let u = (cross(view_up, &w)).unit_vector();
        let v = cross(&w, &u);

        let mut result = Camera {
            origin: *look_from,
            horizontal: focus_distance * viewport_width * u,
            vertical: focus_distance * viewport_height * v,
            u,
            v,
            w,
            lower_left_corner: Vec3::default(),
            lens_radius: aperture / 2.0,
        };

        result.lower_left_corner =
            result.origin - result.horizontal / 2.0 - result.vertical / 2.0 - focus_distance * w;

        return result;
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let random_vector = self.lens_radius * Vec3::random_in_unit_sphere();
        let offset = self.u * random_vector.x() + self.v * random_vector.y();
        return Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let aspect_ratio = 16.0 / 9.0;
        let camera = Camera::new(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(0.0, 0.0, 1.0),
            &Vec3::new(0.0, 1.0, 0.0),
            90.0,
            aspect_ratio,
            2.0,
            1.0,
        );

        let expected_origin = Point3::default();
        let expected_llc = Point3::new(-aspect_ratio, -1.0, -1.0);
        let expected_horizontal = Vec3::new(aspect_ratio * 2.0, 0.0, 0.0);
        let expected_vertical = Vec3::new(0.0, 2.0, 0.0);

        assert_eq!(expected_origin, camera.origin);
        assert_eq!(expected_llc, camera.lower_left_corner);
        assert_eq!(expected_horizontal, camera.horizontal);
        assert_eq!(expected_vertical, camera.vertical);
    }

    #[test]
    fn get_ray() {
        let aspect_ratio = 16.0 / 9.0;
        let camera = Camera::new(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(0.0, 0.0, 1.0),
            &Vec3::new(0.0, 1.0, 0.0),
            90.0,
            aspect_ratio,
            2.0,
            1.0,
        );
        let expected_ray = Ray::new(Point3::default(), Vec3::new(-aspect_ratio, -1.0, -1.0));
        assert_eq!(expected_ray, camera.get_ray(0.0, 0.0));
    }
}
