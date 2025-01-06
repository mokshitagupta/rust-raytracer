use crate::{
    cmp, cross, deg2rad, rand_norm, rand_outside, rand_unit_vector, unit_vector, write_clr, Color3,
    HitRecord, Hittable, Interval, Point3, Ray, Vec3, INFINTY,
};

pub struct Camera {
    aspect_ratio: f64,
    w: u64,
    h: u64,
    center: Point3,
    pixel00: Point3,
    pixelDeltau: Vec3,
    pixelDeltav: Vec3,
    samplesPerPixel: u64,
    pixelSamplesScale: f64,
    maxDepth: u32,
    fov: f64,
    w_a: Vec3,
    u_a: Vec3,
    v_a: Vec3,
}

impl Camera {
    pub fn render(&self, world: &mut (impl Hittable)) {
        print!("P3\n{} {}\n255\n", self.w, self.h);
        for i in 0..self.h {
            eprintln!("REMAINING LINES === {}", self.h - i);
            for j in 0..self.w {
                // eprintln!("   REMAINING PIX === {}", self.w - j);
                let mut clr = Color3::new();
                for s in 0..self.samplesPerPixel {
                    let r = self.get_ray(i, j);
                    let pixelClr: Color3 = self.ray_color(r, self.maxDepth, world);
                    clr += pixelClr;
                }
                write_clr(clr * self.pixelSamplesScale, false);
            }
        }
    }

    pub fn get_ray(&self, i: u64, j: u64) -> Ray {
        let offset = self.sample_square();
        let pixelCenter = self.pixel00
            + ((j as f64 + offset.x()) * self.pixelDeltau)
            + ((i as f64 + offset.y()) * self.pixelDeltav);
        let rayDir = pixelCenter - self.center;
        return Ray::from(self.center, rayDir);
    }

    pub fn new(
        aspect_ratio: f64,
        w: u64,
        samplesPerPixel: u64,
        maxDepth: u32,
        fov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
    ) -> Camera {
        let h = cmp::max((w as f64 / aspect_ratio) as u64, 1);

        let camCenter: Point3 = lookfrom;
        let focalLength = (lookfrom - lookat).length();
        let w_a = unit_vector(lookfrom - lookat);
        let u_a = unit_vector(cross(vup, w_a));
        let v_a = cross(w_a, u_a);
        let fov_angle = f64::tan(deg2rad(fov / 2.0));
        let vpHeight = 2.0 * focalLength * fov_angle;
        let vpWidth = vpHeight * (w as f64 / h as f64);
        let vpu = vpWidth * u_a;
        let vpv = vpHeight * (-v_a);

        let pixelDeltau = vpu / (w as f64);
        let pixelDeltav = vpv / (h as f64);
        let vpUpperLeft = camCenter - (focalLength * w_a) - (vpu / 2.0) - (vpv / 2.0);
        let pixel00 = vpUpperLeft + 0.5 * (pixelDeltau + pixelDeltav);
        Camera {
            aspect_ratio,
            w,
            h,
            center: camCenter,
            pixel00,
            pixelDeltau,
            pixelDeltav,
            samplesPerPixel,
            pixelSamplesScale: 1.0 / (samplesPerPixel as f64),
            maxDepth,
            fov,
            w_a,
            u_a,
            v_a,
        }
    }

    pub fn ray_color(&self, r: Ray, depth: u32, world: &mut impl Hittable) -> Color3 {
        if depth <= 0 {
            return Color3::new();
        }
        let mut rec = HitRecord::new();
        if world.hit(r, Interval::from(0.001, INFINTY), &mut rec) {
            let mut attenuation = Color3::new();
            let mut scattered = Ray::from(Point3::new(), Vec3::new());
            if rec
                .mat
                .borrow_mut()
                .scatter(r, &mut rec.clone(), &mut attenuation, &mut scattered)
            {
                return attenuation * self.ray_color(scattered, depth - 1, world);
            }
            return Color3::new();
        }
        let uDir: Vec3 = unit_vector(r.direction());
        let a = 0.5 * (uDir.y() + 1.0);
        return (1.0 - a) * Color3::from(1.0, 1.0, 1.0) + a * Color3::from(0.5, 0.7, 1.0);
    }

    pub fn sample_square(&self) -> Vec3 {
        Vec3::from(rand_norm() - 0.5, rand_norm() - 0.5, 0.0)
    }
}
