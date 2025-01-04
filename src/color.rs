use crate::intervals::Interval;
pub use crate::vec3::Vec3 as Color3;

#[inline(always)]
pub fn lin2gamma(p: f64) -> f64 {
    if p > 0.0 {
        return p.sqrt();
    } else {
        return 0.0;
    }
}

pub fn write_clr(pixel: Color3, stderr: bool) {
    let intensity: Interval = Interval::from(0.0, 0.999);
    let mut r = pixel.x();
    let mut g = pixel.y();
    let mut b = pixel.z();
    r = lin2gamma(r);
    g = lin2gamma(g);
    b = lin2gamma(b);
    r = (intensity.clamp(r) * (255.0 as f64));
    g = (intensity.clamp(g) * (255.0 as f64));
    b = (intensity.clamp(b) * (255.0 as f64));

    if stderr {
        eprintln!("{r} {g} {b}");
    } else {
        println!("{r} {g} {b}")
    }
    // content.push_str(format!("{} {} {}\n", r, g, 0).as_str());
}
