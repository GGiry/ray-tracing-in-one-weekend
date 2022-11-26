use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

use crate::utils::{random_f64, random_f64_range};

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_f64(),
            y: random_f64(),
            z: random_f64(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_f64_range(min, max),
            y: random_f64_range(min, max),
            z: random_f64_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let result = Vec3::random_range(-1.0, 1.0);
            if result.length_squared() >= 1.0 {
                continue;
            }
            return result;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    // Return true if the vector is close to zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        let delta = 1e-8;
        self.x.abs() < delta && self.y.abs() < delta && self.z.abs() < delta
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - 2.0 * dot(self, normal) * *normal
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = dot(&(-(*self)), normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * *normal;
        r_out_perp + r_out_parallel
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

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl PartialEq<Self> for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Vec3 {}

impl Sum<Vec3> for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Self {
        let mut result = Vec3::default();
        for vec in iter {
            result += vec;
        }
        result
    }
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3 {
        x: lhs.y * rhs.z - lhs.z * rhs.y,
        y: lhs.z * rhs.x - lhs.x * rhs.z,
        z: lhs.x * rhs.y - lhs.y * rhs.x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            z: 3.0,
        };

        assert_eq!(1.0, vec.x());
        assert_eq!(2.0, vec.y());
        assert_eq!(3.0, vec.z());
    }

    #[test]
    fn vec3_neg() {
        let vec = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
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
        let vec = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

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
    #[should_panic(expected = "3 is not a valid index")]
    fn vec3_array_access_oob2() {
        let vec = Vec3::default();

        vec[3];
    }

    #[test]
    fn vec3_add_assign() {
        let mut vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec_b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        vec_a += vec_b;

        assert_eq!(5.0, vec_a.x());
        assert_eq!(7.0, vec_a.y());
        assert_eq!(9.0, vec_a.z());
    }

    #[test]
    fn vec3_mul_assign() {
        let mut vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let operand = 2.0;

        vec_a *= operand;

        assert_eq!(2.0, vec_a.x());
        assert_eq!(4.0, vec_a.y());
        assert_eq!(6.0, vec_a.z());
    }

    #[test]
    fn vec3_div_assign() {
        let mut vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let operand = 2.0;

        vec_a /= operand;

        assert_eq!(0.5, vec_a.x());
        assert_eq!(1.0, vec_a.y());
        assert_eq!(1.5, vec_a.z());
    }

    #[test]
    fn vec3_length_squared() {
        let vec_default = Vec3::default();
        let vec_ones = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let vec = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        assert_eq!(0.0, vec_default.length_squared());
        assert_eq!(3.0, vec_ones.length_squared());
        assert_eq!(29.0, vec.length_squared());
    }

    #[test]
    fn vec3_length() {
        let vec_default = Vec3::default();
        let vec_ones = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let vec = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        assert_eq!(0.0_f64.sqrt(), vec_default.length());
        assert_eq!(3.0_f64.sqrt(), vec_ones.length());
        assert_eq!(29.0_f64.sqrt(), vec.length());
    }

    #[test]
    fn vec3_display() {
        let vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec_b = Vec3 {
            x: 0.5,
            y: 1.9,
            z: 10.99,
        };

        assert_eq!(format!("{vec_a}"), "1 2 3");
        assert_eq!(format!("{vec_b}"), "0.5 1.9 10.99");
    }

    #[test]
    fn vec3_add() {
        let vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec_b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        let vec_c = vec_a + vec_b;

        assert_eq!(1.0, vec_a.x());
        assert_eq!(2.0, vec_a.y());
        assert_eq!(3.0, vec_a.z());

        assert_eq!(4.0, vec_b.x());
        assert_eq!(5.0, vec_b.y());
        assert_eq!(6.0, vec_b.z());

        assert_eq!(5.0, vec_c.x());
        assert_eq!(7.0, vec_c.y());
        assert_eq!(9.0, vec_c.z());
    }

    #[test]
    fn vec3_sub() {
        let vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec_b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        let vec_c = vec_a - vec_b;

        assert_eq!(1.0, vec_a.x());
        assert_eq!(2.0, vec_a.y());
        assert_eq!(3.0, vec_a.z());

        assert_eq!(4.0, vec_b.x());
        assert_eq!(5.0, vec_b.y());
        assert_eq!(6.0, vec_b.z());

        assert_eq!(-3.0, vec_c.x());
        assert_eq!(-3.0, vec_c.y());
        assert_eq!(-3.0, vec_c.z());
    }

    #[test]
    fn vec3_mul_vec3() {
        let vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec_b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        let vec_c = vec_a * vec_b;

        assert_eq!(1.0, vec_a.x());
        assert_eq!(2.0, vec_a.y());
        assert_eq!(3.0, vec_a.z());

        assert_eq!(4.0, vec_b.x());
        assert_eq!(5.0, vec_b.y());
        assert_eq!(6.0, vec_b.z());

        assert_eq!(4.0, vec_c.x());
        assert_eq!(10.0, vec_c.y());
        assert_eq!(18.0, vec_c.z());
    }

    #[test]
    fn f64_mul_vec3() {
        let vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let vec_c = 3.0 * vec_a;

        assert_eq!(1.0, vec_a.x());
        assert_eq!(2.0, vec_a.y());
        assert_eq!(3.0, vec_a.z());

        assert_eq!(3.0, vec_c.x());
        assert_eq!(6.0, vec_c.y());
        assert_eq!(9.0, vec_c.z());
    }

    #[test]
    fn vec3_mul_f64() {
        let vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let vec_c = vec_a * 3.0;

        assert_eq!(1.0, vec_a.x());
        assert_eq!(2.0, vec_a.y());
        assert_eq!(3.0, vec_a.z());

        assert_eq!(3.0, vec_c.x());
        assert_eq!(6.0, vec_c.y());
        assert_eq!(9.0, vec_c.z());
    }

    #[test]
    fn vec3_div_f64() {
        let vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let vec_c = vec_a / 2.0;

        assert_eq!(1.0, vec_a.x());
        assert_eq!(2.0, vec_a.y());
        assert_eq!(3.0, vec_a.z());

        assert_eq!(0.5, vec_c.x());
        assert_eq!(1.0, vec_c.y());
        assert_eq!(1.5, vec_c.z());
    }

    #[test]
    fn vec3_dot_vec3() {
        let vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec_b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        let dot_product = dot(&vec_a, &vec_b);

        assert_eq!(1.0, vec_a.x());
        assert_eq!(2.0, vec_a.y());
        assert_eq!(3.0, vec_a.z());

        assert_eq!(4.0, vec_b.x());
        assert_eq!(5.0, vec_b.y());
        assert_eq!(6.0, vec_b.z());

        assert_eq!(32.0, dot_product);
    }

    #[test]
    fn vec3_cross_vec3() {
        let vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec_b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        let vec_c = cross(&vec_a, &vec_b);

        assert_eq!(1.0, vec_a.x());
        assert_eq!(2.0, vec_a.y());
        assert_eq!(3.0, vec_a.z());

        assert_eq!(4.0, vec_b.x());
        assert_eq!(5.0, vec_b.y());
        assert_eq!(6.0, vec_b.z());

        assert_eq!(-3.0, vec_c.x());
        assert_eq!(6.0, vec_c.y());
        assert_eq!(-3.0, vec_c.z());
    }

    #[test]
    fn unit_vector() {
        let vec = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        let unit_vector = vec.unit_vector();

        assert_eq!(4.0, vec.x());
        assert_eq!(5.0, vec.y());
        assert_eq!(6.0, vec.z());

        assert_eq!(4.0 / 77.0_f64.sqrt(), unit_vector.x());
        assert_eq!(5.0 / 77.0_f64.sqrt(), unit_vector.y());
        assert_eq!(6.0 / 77.0_f64.sqrt(), unit_vector.z());
    }

    #[test]
    fn vector_cmp() {
        let vec_a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let vec_b = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(vec_a, vec_b);
    }

    #[test]
    fn test_near_zero() {
        assert!(Vec3::default().near_zero());
        assert!(Vec3::new(0.000000001, 0.0, 0.0).near_zero());
        assert!(!Vec3::new(1.0, 1.0, 1.0).near_zero());
        assert!(!Vec3::new(0.00000001, 0.0, 0.0).near_zero());
    }

    #[test]
    fn test_sum() {
        let sum: Vec3 = (0..3)
            .map(|index| Vec3::new(index as f64, 2.0 * index as f64, 3.0 * index as f64))
            .sum();

        assert_eq!(sum, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_reflect() {
        let vec = Vec3 {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        };

        let normal = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let expected = Vec3 {
            x: -1.0,
            y: -1.0,
            z: 0.0,
        };

        assert_eq!(expected, vec.reflect(&normal));
    }

    #[test]
    fn test_refract() {
        let vec = Vec3 {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        };

        let normal = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let expected = Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        };

        assert_eq!(expected, vec.refract(&normal, 1.0));
    }
}
