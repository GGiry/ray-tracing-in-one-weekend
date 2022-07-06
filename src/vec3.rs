use derivative::Derivative;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, DivAssign, Index, Mul, MulAssign, Neg, Sub};

#[derive(Copy, Clone, Derivative)]
#[derivative(Default)]
pub struct Vec3 {
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

    fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    fn length(&self) -> f64 {
        return self.length_squared().sqrt();
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
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
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
}
