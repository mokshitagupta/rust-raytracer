use crate::intervals::Interval;
pub use crate::vec3::Vec3 as Color3;

pub fn write_clr(pixel: Color3, stderr: bool) {
    let intensity: Interval = Interval::from(0.0, 0.999);
    let r = (intensity.clamp(pixel.x()) * (255.0 as f64));
    let g = (intensity.clamp(pixel.y()) * (255.0 as f64));
    let b = (intensity.clamp(pixel.z()) * (255.0 as f64));

    if stderr {
        eprintln!("{r} {g} {b}");
    } else {
        println!("{r} {g} {b}")
    }
    // content.push_str(format!("{} {} {}\n", r, g, 0).as_str());
}
