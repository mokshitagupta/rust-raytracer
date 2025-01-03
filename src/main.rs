use core::f64;
// use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::{cmp, fs::File, io::Write};
mod color;
use color::*;
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
    fn hit(&mut self, r: Ray, tmin: f64, tmax: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&mut self, r: Ray, tmin: f64, tmax: f64, rec: &mut HitRecord) -> bool {
        let uDir: Vec3 = unit_vector(r.direction());
        // let center = Vec3::from(0.0, 0.0, -1.0);
        let cmq = self.center - r.origin();
        let ai = r.direction().length_squared();
        let h = dot(r.direction(), cmq);
        // let b = dot(-2.0 * uDir, cmq);
        // let radius = 0.5;
        let c = cmq.length_squared() - (self.radius * self.radius);
        //(b^2 - 4ac ) <- sqrt
        let det_in = (h * h) - (ai * c);

        // println!("{uDir:?} {r:?}");
        if det_in < 0.0 {
            return false;
        } else {
            let sqrtd = det_in.sqrt();
            let mut quad_form = (h - sqrtd) / (ai);
            if quad_form <= tmin || quad_form >= tmax {
                quad_form = (h + sqrtd) / (ai);
                if quad_form <= tmin || quad_form >= tmax {
                    return false;
                }
            }

            rec.t = quad_form;
            rec.p = r.at(rec.t);
            let out_norm = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, out_norm);
            // let p = r.origin() + (quad_form * r.direction());
            return true;
            // let N = unit_vector(p - self.center);
            // return 0.5 * Color3::from(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
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
    fn hit(&mut self, r: Ray, tmin: f64, tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_any = false;
        let mut closest = tmax;

        for i in self.objects.iter() {
            if i.borrow_mut().hit(r, tmin, closest, &mut temp_rec) {
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

fn ray_color(r: Ray, world: &mut (impl Hittable + List)) -> Color3 {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color3::from(1.0, 1.0, 1.0));
    }
    let uDir: Vec3 = unit_vector(r.direction());
    let a = 0.5 * (uDir.y() + 1.0);
    return (1.0 - a) * Color3::from(1.0, 1.0, 1.0) + a * Color3::from(0.5, 0.7, 1.0);

    // let center = Vec3::from(0.0, 0.0, -1.0);
    // let cmq = center - r.origin();
    // let ai = uDir.length_squared();
    // let b = dot(-2.0 * uDir, cmq);
    // let radius = 0.5;
    // let c = dot(cmq, cmq) - (radius * radius);
    // //(b^2 - 4ac ) <- sqrt
    // let det_in = (b * b) - (4.0 * ai * c);

    // // println!("{uDir:?} {r:?}");
    // if det_in < 0.0 {
    //     return clr;
    // } else {
    //     let quad_form = (-b - det_in.sqrt()) / (2.0 * ai);
    //     let p = r.origin() + (quad_form * r.direction());
    //     let N = unit_vector(p - center);
    //     return 0.5 * Color3::from(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    // };
}

fn generate_img(w: u64) {
    let aspectRatio: f64 = 16.0 / 9.0;
    let h = cmp::max((w as f64 / aspectRatio) as u64, 1);

    let mut world = HittableList::new();
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(0.0, 0.0, -1.0),
        0.5,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
    ))));

    let focalLength = 1.0;
    let vpHeight = 2.0;
    let vpWidth = vpHeight * (w as f64 / h as f64);
    let camCenter: Point3 = Point3::new();
    let vpu = Vec3::from(vpWidth, 0.0, 0.0);
    let vpv = Vec3::from(0.0, -vpHeight, 0.0);
    let pixelDeltau = vpu / (w as f64);
    let pixelDeltav = vpv / (h as f64);

    let vpUpperLeft = camCenter - Vec3::from(0.0, 0.0, focalLength) - (vpu / 2.0) - (vpv / 2.0);
    let pixel00 = vpUpperLeft + 0.5 * (pixelDeltau + pixelDeltav);
    print!("P3\n{w} {h}\n255\n");
    for i in 0..h {
        for j in 0..w {
            let pixelCenter = pixel00 + (j as f64 * pixelDeltau) + (i as f64 * pixelDeltav);
            let rayDir = pixelCenter - camCenter;
            let r = Ray::from(camCenter, rayDir);
            let pixelClr: Color3 = ray_color(r, &mut world);
            write_clr(pixelClr, false);
        }
    }
}

fn main() {
    generate_img(400);
}
