pub use crate::vec3::Vec3 as Color3;

pub fn write_clr(pixel: Color3, stderr: bool) {
    let r = (pixel.x() * (255.0 as f64));
    let g = (pixel.y() * (255.0 as f64));
    let b = (pixel.z() * (255.0 as f64));

    if stderr {
        eprintln!("{r} {g} {b}");
    } else {
        println!("{r} {g} {b}")
    }
    // content.push_str(format!("{} {} {}\n", r, g, 0).as_str());
}
