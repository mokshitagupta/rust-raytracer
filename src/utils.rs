pub const INFINTY: f64 = f64::INFINITY;
pub const NEG_INFINTY: f64 = -f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

use rand::prelude::*;

pub fn deg2rad(deg: f64) -> f64 {
    return deg * PI / 180.0;
}

pub fn rand_norm() -> f64 {
    return rand::thread_rng().gen();
}

pub fn rand_from(min: f64, max: f64) -> f64 {
    return min + (max - min) * rand_norm();
}
