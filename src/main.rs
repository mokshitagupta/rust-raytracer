use std::{cmp, fs::File, io::Write};
mod color;
use color::*;
mod ray;
use ray::*;
mod vec3;
use vec3::*;

fn ray_color(r: Ray) -> Color3 {
    let uDir: Vec3 = unit_vector(r.direction());
    let center = Vec3::from(0.0, 0.0, -1.0);
    let cmq = center - r.origin();
    let ai = uDir.length_squared();
    let b = dot(-2.0 * uDir, cmq);
    let radius = 0.5;
    let c = dot(cmq, cmq) - (radius * radius);
    //(b^2 - 4ac ) <- sqrt
    let det_in = (b * b) - (4.0 * ai * c);

    // println!("{uDir:?} {r:?}");
    let a = 0.5 * (uDir.y() + 1.0);
    let clr = (1.0 - a) * Color3::from(1.0, 1.0, 1.0) + a * Color3::from(0.5, 0.7, 1.0);
    if det_in < 0.0 {
        return clr;
    } else {
        let quad_form = (-b - det_in.sqrt()) / (2.0 * ai);
        let p = r.origin() + (quad_form * r.direction());
        let N = unit_vector(p - center);
        return 0.5 * Color3::from(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    };
}

fn generate_img(w: u64) {
    let aspectRatio: f64 = 16.0 / 9.0;
    let h = cmp::max((w as f64 / aspectRatio) as u64, 1);

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
            let pixelClr: Color3 = ray_color(r);
            write_clr(pixelClr, false);
        }
    }
}

fn main() {
    generate_img(400);
}
