use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

impl Vec3 {
    pub fn new(i: f32, j: f32, k: f32) -> Self {
        Self { i, j, k }
    }

    pub fn length_squared(&self) -> f32 {
        self.i.powi(2) + self.j.powi(2) + self.k.powi(2)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Self {
        self / self.length()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3::new(
            self.j * rhs.k - self.k * rhs.j,
            self.k * rhs.i - self.i * rhs.k,
            self.i * rhs.j - self.j * rhs.i,
        )
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.i * rhs.i + self.j * rhs.j + self.k * rhs.k
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.i, self.j, self.k)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from((i, j, k): (f32, f32, f32)) -> Self {
        Self { i, j, k }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.i,
            1 => &self.j,
            2 => &self.k,
            n => panic!(
                "index out of bounds, expected `0`, `1`, or `2` found `{}`",
                n
            ),
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.i + rhs.i, self.j + rhs.j, self.k + rhs.k)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.i += rhs.i;
        self.j += rhs.j;
        self.k += rhs.k;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.i - rhs.i, self.j - rhs.j, self.k - rhs.k)
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.i - rhs.i, self.j - rhs.j, self.k - rhs.k)
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Self) -> Self::Output {
        Vec3::new(self.i - rhs.i, self.j - rhs.j, self.k - rhs.k)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.i -= rhs.i;
        self.j -= rhs.j;
        self.k -= rhs.k;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new(-self.i, -self.j, -self.k)
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length_squared().partial_cmp(&other.length_squared())
    }
}

macro_rules! vec3_scalar_mul {
    ($rhs:ty) => {
        impl Div<$rhs> for Vec3 {
            type Output = Vec3;

            fn div(self, rhs: $rhs) -> Self::Output {
                Vec3::new(
                    self.i / (rhs as f32),
                    self.j / (rhs as f32),
                    self.k / (rhs as f32),
                )
            }
        }

        impl Div<$rhs> for &Vec3 {
            type Output = Vec3;

            fn div(self, rhs: $rhs) -> Self::Output {
                Vec3::new(
                    self.i / (rhs as f32),
                    self.j / (rhs as f32),
                    self.k / (rhs as f32),
                )
            }
        }

        impl Div<Vec3> for $rhs {
            type Output = Vec3;

            fn div(self, rhs: Vec3) -> Self::Output {
                Vec3::new(
                    self as f32 / rhs.i,
                    self as f32 / rhs.j,
                    self as f32 / rhs.k,
                )
            }
        }

        impl DivAssign<$rhs> for Vec3 {
            fn div_assign(&mut self, rhs: $rhs) {
                self.i /= (rhs as f32);
                self.j /= (rhs as f32);
                self.k /= (rhs as f32);
            }
        }

        impl Mul<$rhs> for Vec3 {
            type Output = Vec3;

            fn mul(self, rhs: $rhs) -> Self::Output {
                Vec3::new(
                    self.i * (rhs as f32),
                    self.j * (rhs as f32),
                    self.k * (rhs as f32),
                )
            }
        }

        impl Mul<$rhs> for &Vec3 {
            type Output = Vec3;

            fn mul(self, rhs: $rhs) -> Self::Output {
                Vec3::new(
                    self.i * (rhs as f32),
                    self.j * (rhs as f32),
                    self.k * (rhs as f32),
                )
            }
        }

        impl Mul<Vec3> for $rhs {
            type Output = Vec3;

            fn mul(self, rhs: Vec3) -> Self::Output {
                Vec3::new(
                    self as f32 * rhs.i,
                    self as f32 * rhs.j,
                    self as f32 * rhs.k,
                )
            }
        }

        impl MulAssign<$rhs> for Vec3 {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.i *= (rhs as f32);
                self.j *= (rhs as f32);
                self.k *= (rhs as f32);
            }
        }
    };
    ($($rhs:ty),+) => {
        $(
            vec3_scalar_mul!($rhs);
        )+
    }
}

vec3_scalar_mul!(f32, f64, i32, i64, u32, u64);
