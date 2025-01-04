use crate::vec3::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn from(orig: Point3, dir: Vec3) -> Ray {
        Ray {
            orig: orig,
            direction: dir,
        }
    }

    pub fn set(&mut self, orig: Point3, dir: Vec3) {
        self.orig = orig;
        self.direction = dir;
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.direction
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
