use std::fs::File;
use std::io::Write;
use std::ops::{AddAssign, Index, MulAssign, Neg};

use derivative::Derivative;

#[derive(Copy, Clone, Derivative)]
#[derivative(Default)]
struct Vec3 {
    #[derivative(Default(value = "0.0"))]
    x: f64,
    #[derivative(Default(value = "0.0"))]
    y: f64,
    #[derivative(Default(value = "0.0"))]
    z: f64,
}

impl Vec3 {
    fn x(&self) -> f64 {
        return self.x;
    }

    fn y(&self) -> f64 {
        return self.y;
    }

    fn z(&self) -> f64 {
        return self.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<i32> for Vec3 {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {

        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("{index} is not a valid index"),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

fn create_gradient(path: &str) {
    let mut file = File::create(path).unwrap();

    let image_width = 256;
    let image_height = 256;

    file.write_all(b"P3\n").expect("Unable to write data");
    file.write_all(format!("{image_width} {image_height}\n").as_bytes())
        .expect("Unable to write data");
    file.write_all(b"255\n").expect("Unable to write data");

    for index_height in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {index_height}");
        for index_width in 0..image_width {
            let red = index_width as f64 / ((image_width - 1) as f64);
            let green = index_height as f64 / ((image_height - 1) as f64);
            let blue = 0.25;

            let int_red: i32 = (255.999 * red) as i32;
            let int_green: i32 = (255.999 * green) as i32;
            let int_blue: i32 = (255.999 * blue) as i32;

            file.write_all(format!("{int_red} {int_green} {int_blue}\n").as_bytes())
                .expect("Unable to write data");
        }
    }
    eprintln!("Done.");
}

fn main() {
    create_gradient("result.ppm");
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_diff::diff;

    #[test]
    fn gradient_ppm() {
        let result_path = "result_gradient.ppm";
        create_gradient(result_path);
        assert_eq!(true, diff("expected_gradient.ppm", result_path));
    }

    #[test]
    fn vec3_default() {
        let vec = Vec3::default();

        assert_eq!(0.0, vec.x());
        assert_eq!(0.0, vec.y());
        assert_eq!(0.0, vec.z());
    }

    #[test]
    fn vec3_assign() {
        let vec = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0
        };

        assert_eq!(1.0, vec.x());
        assert_eq!(2.0, vec.y());
        assert_eq!(3.0, vec.z());
    }

    #[test]
    fn vec3_neg() {
        let vec = Vec3 {x: 1.0, y: 2.0, z: 3.0 };
        let vec_neg = -vec;

        assert_eq!(-1.0, vec_neg.x());
        assert_eq!(-2.0, vec_neg.y());
        assert_eq!(-3.0, vec_neg.z());

        assert_eq!(1.0, vec.x());
        assert_eq!(2.0, vec.y());
        assert_eq!(3.0, vec.z());
    }

    #[test]
    fn vec3_array_access() {
        let vec = Vec3 {x: 1.0, y: 2.0, z: 3.0 };

        assert_eq!(1.0, vec[0]);
        assert_eq!(2.0, vec[1]);
        assert_eq!(3.0, vec[2]);
    }

    #[test]
    #[should_panic(expected = "-1 is not a valid index")]
    fn vec3_array_access_oob() {
        let vec = Vec3::default();
        vec[-1];
    }

    #[test]
    #[should_panic(expected ="3 is not a valid index")]
    fn vec3_array_access_oob2() {
        let vec = Vec3::default();
        vec[3];
    }

    #[test]
    fn vec3_add_assign() {
        let mut vec_a = Vec3 {x: 1.0, y: 2.0, z: 3.0 };
        let vec_b = Vec3 {x: 4.0, y: 5.0, z: 6.0 };

        vec_a += vec_b;

        assert_eq!(5.0, vec_a.x());
        assert_eq!(7.0, vec_a.y());
        assert_eq!(9.0, vec_a.z());
    }

    #[test]
    fn vec3_mul_assign() {
        let mut vec_a = Vec3 {x: 1.0, y: 2.0, z: 3.0 };
        let operand = 2.0;

        vec_a *= operand;

        assert_eq!(2.0, vec_a.x());
        assert_eq!(4.0, vec_a.y());
        assert_eq!(6.0, vec_a.z());
    }
}
