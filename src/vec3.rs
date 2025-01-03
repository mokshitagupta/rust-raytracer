use std::ops::{self};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub use Vec3 as Point3;

use crate::{rand_from, rand_norm};

impl Vec3 {
    pub fn new() -> Self {
        return Self { e: [0.0, 0.0, 0.0] };
    }

    pub fn from(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { e: [x, y, z] };
    }

    pub fn x(&self) -> f64 {
        return self.e[0];
    }
    pub fn y(&self) -> f64 {
        return self.e[1];
    }
    pub fn z(&self) -> f64 {
        return self.e[2];
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn rand_norm() -> Vec3 {
        Vec3::from(rand_norm(), rand_norm(), rand_norm())
    }
    pub fn rand_from(min: f64, max: f64) -> Vec3 {
        Vec3::from(
            rand_from(min, max),
            rand_from(min, max),
            rand_from(min, max),
        )
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::from(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::from(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::from(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Vec3 {
        return Vec3::from(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs);
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Vec3 {
        return Vec3::from(self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]);
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, rhs: f64) -> Vec3 {
        return self * (1.0 / rhs);
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] *= (1.0 / rhs);
        self.e[1] *= (1.0 / rhs);
        self.e[2] *= (1.0 / rhs);
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    #[inline(always)]
    fn index(&self, index: usize) -> &f64 {
        return &self.e[index];
    }
}

impl ops::IndexMut<usize> for Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        return &mut self.e[index];
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn neg(self) -> Vec3 {
        return Vec3::from(-self.e[0], -self.e[1], -self.e[2]);
    }
}

#[inline(always)]
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    return u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2];
}

#[inline(always)]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::from(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

#[inline(always)]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[inline(always)]
pub fn rand_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::rand_from(-1.0, 1.0);
        let lsq = p.length_squared();
        // eprintln!("     iter {}", lsq);
        if lsq <= 1.0 && 1e-160 < lsq {
            // eprintln!("solution {:?} {}", p, lsq);
            return p / lsq.sqrt();
        }
    }
}

#[inline(always)]
pub fn rand_outside(norm: Vec3) -> Vec3 {
    let on_sphere = rand_unit_vector();
    if dot(on_sphere, norm) > 0.0 {
        return on_sphere;
    } else {
        return -on_sphere;
    }
}
