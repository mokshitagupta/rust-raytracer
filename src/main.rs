use core::f64;
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

#[derive(Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    mat: Rc<RefCell<dyn Material>>,
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
            mat: Rc::new(RefCell::new(Lambertian::from(Color3::new()))),
        }
    }

    pub fn copy(&mut self, rec: &HitRecord) {
        self.p = rec.p;
        self.front_face = rec.front_face;
        self.normal = rec.normal;
        self.t = rec.t;
        self.mat = rec.mat.clone();
    }
}

pub trait Hittable {
    fn hit(&mut self, r: Ray, ray_root: Interval, rec: &mut HitRecord) -> bool;
}

pub trait Material {
    fn scatter(
        &mut self,
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color3,
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color3,
    fuzz: f64,
}

#[derive(Debug)]
pub struct Diaelectric {
    refractive_index: f64,
}

impl Material for Diaelectric {
    fn scatter(
        &mut self,
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool {
        let mut rind = self.refractive_index;
        if rec.front_face {
            rind = 1.0 / self.refractive_index;
        }

        let cos_theta = dot(-unit_vector(r_in.direction()), rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let scatter;
        if rind * sin_theta > 1.0 || (Diaelectric::reflectance(cos_theta, rind) > rand_norm()) {
            // reflect
            scatter = reflect(unit_vector(r_in.direction()), rec.normal);
        } else {
            scatter = refract(rind, unit_vector(r_in.direction()), rec.normal);
        }
        scattered.set(rec.p, scatter);
        attenuation.set(1.0, 1.0, 1.0);
        true
    }
}

impl Diaelectric {
    pub fn from(ind: f64) -> impl Material {
        Diaelectric {
            refractive_index: ind,
        }
    }

    pub fn reflectance(cosine: f64, n1: f64) -> f64 {
        let mut r0 = (1.0 - n1) / (1.0 + n1);
        r0 *= r0;
        r0 + ((1.0 - r0) * (1.0 - cosine).powi(5))
    }
}

impl Material for Lambertian {
    fn scatter(
        &mut self,
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool {
        let mut dir = rec.normal + rand_unit_vector();
        if dir.near_zero() {
            dir = rec.normal;
        }
        scattered.set(rec.p, dir);
        attenuation.copy(self.albedo);
        // eprintln!("{attenuation:?} {:?}", self.albedo);
        true
    }
}

impl Material for Metal {
    fn scatter(
        &mut self,
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(r_in.direction(), rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * rand_unit_vector());
        scattered.set(rec.p, reflected);
        attenuation.copy(self.albedo);
        dot(reflected, rec.normal) > 0.0
    }
}

impl Metal {
    fn from(albedo: Color3, fuzz: f64) -> impl Material {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Lambertian {
    fn from(albedo: Color3) -> impl Material {
        Lambertian { albedo }
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<RefCell<dyn Material>>,
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
            rec.mat = self.mat.clone();
            return true;
        };
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<RefCell<dyn Material>>) -> impl Hittable {
        Sphere {
            center,
            radius,
            mat,
        }
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

fn create3Scene(world: &mut (impl Hittable + List)) {
    let ground_mat = Rc::new(RefCell::new(Lambertian::from(Color3::from(0.8, 0.8, 0.0))));
    let center_mat = Rc::new(RefCell::new(Lambertian::from(Color3::from(0.1, 0.2, 0.5))));
    // let left_mat = Rc::new(RefCell::new(Metal::from(Color3::from(0.8, 0.8, 0.8), 0.3)));
    let left_mat = Rc::new(RefCell::new(Diaelectric::from(1.50)));
    let bubble_mat = Rc::new(RefCell::new(Diaelectric::from(1.00 / 1.50)));
    let right_mat = Rc::new(RefCell::new(Metal::from(Color3::from(0.8, 0.6, 0.2), 1.0)));

    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
        ground_mat,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(0.0, 0.0, -1.2),
        0.5,
        center_mat,
    ))));

    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(-1.0, 0.0, -1.0),
        0.5,
        left_mat,
    ))));

    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(-1.0, 0.0, -1.0),
        0.4,
        bubble_mat,
    ))));

    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(1.0, 0.0, -1.0),
        0.5,
        right_mat,
    ))));
}

fn createFOVScene(world: &mut (impl Hittable + List)) {
    let r = f64::cos(PI / 4.0);
    let ground_mat = Rc::new(RefCell::new(Lambertian::from(Color3::from(1.0, 0.0, 0.0))));
    let center_mat = Rc::new(RefCell::new(Lambertian::from(Color3::from(0.0, 1.0, 0.0))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(-r, 0.0, -1.0),
        r,
        ground_mat,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(r, 0.0, -1.0),
        r,
        center_mat,
    ))));
}

fn createFinalScene(world: &mut (impl Hittable + List)) {
    let ground_mat = Rc::new(RefCell::new(Lambertian::from(Color3::from(0.8, 0.8, 0.0))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::from(0.0, -1000.0, -0.0),
        1000.0,
        ground_mat,
    ))));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::from(
                a as f64 + 0.9 * rand_norm(),
                0.2,
                b as f64 + 0.9 * rand_norm(),
            );
            let prob = rand_norm();

            if (center - Point3::from(4.0, 0.2, 0.0)).length() > 0.9 {
                // let mat;
                if prob < 0.8 {
                    let albedo = Vec3::rand_norm();
                    let lamb_mat = Rc::new(RefCell::new(Lambertian::from(albedo)));
                    world.add(Rc::new(RefCell::new(Sphere::new(center, 0.2, lamb_mat))));
                } else if prob < 0.95 {
                    let metal_mat = Rc::new(RefCell::new(Metal::from(
                        Vec3::rand_from(0.5, 1.0),
                        rand_from(0.0, 0.5),
                    )));
                    world.add(Rc::new(RefCell::new(Sphere::new(center, 0.2, metal_mat))));
                } else {
                    let dia_mat = Rc::new(RefCell::new(Diaelectric::from(1.50)));
                    world.add(Rc::new(RefCell::new(Sphere::new(center, 0.2, dia_mat))));
                }
            }

            let mat1 = Rc::new(RefCell::new(Diaelectric::from(1.50)));
            world.add(Rc::new(RefCell::new(Sphere::new(
                Point3::from(0.0, 1.0, 0.0),
                1.0,
                mat1,
            ))));

            let mat2 = Rc::new(RefCell::new(Lambertian::from(Vec3::from(0.4, 0.2, 0.1))));
            world.add(Rc::new(RefCell::new(Sphere::new(
                Point3::from(-4.0, 1.0, 0.0),
                1.0,
                mat2,
            ))));

            let mat3 = Rc::new(RefCell::new(Metal::from(Vec3::from(0.7, 0.6, 0.5), 0.0)));
            world.add(Rc::new(RefCell::new(Sphere::new(
                Point3::from(4.0, 1.0, 0.0),
                1.0,
                mat3,
            ))));
        }
    }
}

fn generate_img(w: u64) {
    let aspectRatio: f64 = 16.0 / 9.0;
    let mut world = HittableList::new();
    let fov = 20.0;
    let lookfrom = Vec3::from(13.0, 2.0, 3.0);
    let lookat = Vec3::from(0.0, 0.0, 0.0);
    let vup = Vec3::from(0.0, 1.0, 0.0);
    let defocus_angle = 0.01;
    let focus_dist = 10.0;
    // create3Scene(&mut world);
    createFinalScene(&mut world);
    // createFOVScene(&mut world);
    let camera = Camera::new(
        aspectRatio,
        w,
        100,
        50,
        fov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );
    camera.render(&mut world);
}

fn main() {
    generate_img(400);
}
