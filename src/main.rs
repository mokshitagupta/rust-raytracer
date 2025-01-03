use core::f64;
// use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
mod camera;
use camera::*;
mod color;
mod intervals;
use intervals::*;
mod utils;
use color::*;
use intervals::Interval;
use utils::*;
mod ray;
use ray::*;
mod vec3;
use vec3::*;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, out_norm: Vec3) {
        self.front_face = dot(r.direction(), out_norm) < 0.0;
        if self.front_face {
            self.normal = out_norm;
        } else {
            self.normal = -out_norm;
        }
    }

    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn copy(&mut self, rec: &HitRecord) {
        self.p = rec.p;
        self.front_face = rec.front_face;
        self.normal = rec.normal;
        self.t = rec.t;
    }
}

pub trait Hittable {
    fn hit(&mut self, r: Ray, ray_root: Interval, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&mut self, r: Ray, ray_root: Interval, rec: &mut HitRecord) -> bool {
        let cmq = self.center - r.origin();
        let ai = r.direction().length_squared();
        let h = dot(r.direction(), cmq);
        let c = cmq.length_squared() - (self.radius * self.radius);
        let det_in = (h * h) - (ai * c);

        if det_in < 0.0 {
            return false;
        } else {
            let sqrtd = det_in.sqrt();
            let mut quad_form = (h - sqrtd) / (ai);
            if !ray_root.surrounds(quad_form) {
                quad_form = (h + sqrtd) / (ai);
                if !ray_root.surrounds(quad_form) {
                    return false;
                }
            }

            rec.t = quad_form;
            rec.p = r.at(rec.t);
            let out_norm = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, out_norm);
            return true;
        };
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> impl Hittable {
        Sphere { center, radius }
    }
}

pub struct HittableList {
    objects: Vec<Rc<RefCell<dyn Hittable>>>,
}

impl Hittable for HittableList {
    fn hit(&mut self, r: Ray, ray_root: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_any = false;
        let mut closest = ray_root.max;

        for i in self.objects.iter() {
            if i.borrow_mut()
                .hit(r, Interval::from(ray_root.min, closest), &mut temp_rec)
            {
                hit_any = true;
                closest = temp_rec.t;
                rec.copy(&temp_rec);
            }
        }

        return hit_any;
    }
}

pub trait List {
    fn clear(&mut self);
    fn add(&mut self, el: Rc<RefCell<dyn Hittable>>);
}

impl List for HittableList {
    fn add(&mut self, el: Rc<RefCell<dyn Hittable>>) {
        self.objects.push(el);
    }

    fn clear(&mut self) {
        self.objects.clear();
    }
}

impl HittableList {
    pub fn new() -> impl Hittable + List {
        HittableList {
            objects: Vec::new(),
        }
    }
}

fn generate_img(w: u64) {
    let aspectRatio: f64 = 16.0 / 9.0;
    let mut world = HittableList::new();
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(0.0, 0.0, -1.0),
        0.5,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
    ))));

    let camera = Camera::new(aspectRatio, w, 100);
    camera.render(&mut world);
}

fn main() {
    generate_img(400);
}
