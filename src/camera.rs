use crate::{
    cmp, unit_vector, write_clr, Color3, HitRecord, Hittable, Interval, Point3, Ray, Vec3, INFINTY,
};

pub struct Camera {
    aspect_ratio: f64,
    w: u64,
    h: u64,
    center: Point3,
    pixel00: Point3,
    pixelDeltau: Vec3,
    pixelDeltav: Vec3,
}

impl Camera {
    pub fn render(&self, world: &mut (impl Hittable)) {
        print!("P3\n{} {}\n255\n", self.w, self.h);
        for i in 0..self.h {
            for j in 0..self.w {
                let pixelCenter =
                    self.pixel00 + (j as f64 * self.pixelDeltau) + (i as f64 * self.pixelDeltav);
                let rayDir = pixelCenter - self.center;
                let r = Ray::from(self.center, rayDir);
                let pixelClr: Color3 = self.ray_color(r, world);
                write_clr(pixelClr, false);
            }
        }
    }

    pub fn new(aspect_ratio: f64, w: u64) -> Camera {
        let h = cmp::max((w as f64 / aspect_ratio) as u64, 1);

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
        Camera {
            aspect_ratio,
            w,
            h,
            center: camCenter,
            pixel00,
            pixelDeltau,
            pixelDeltav,
        }
    }

    pub fn ray_color(&self, r: Ray, world: &mut impl Hittable) -> Color3 {
        let mut rec = HitRecord::new();
        if world.hit(r, Interval::from(0.0, INFINTY), &mut rec) {
            return 0.5 * (rec.normal + Color3::from(1.0, 1.0, 1.0));
        }
        let uDir: Vec3 = unit_vector(r.direction());
        let a = 0.5 * (uDir.y() + 1.0);
        return (1.0 - a) * Color3::from(1.0, 1.0, 1.0) + a * Color3::from(0.5, 0.7, 1.0);
    }
}