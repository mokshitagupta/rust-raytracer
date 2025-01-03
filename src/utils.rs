pub const INFINTY: f64 = f64::INFINITY;
pub const NEG_INFINTY: f64 = -f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn deg2rad(deg: f64) -> f64 {
    return deg * PI / 180.0;
}
