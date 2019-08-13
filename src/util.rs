extern crate rand;

use crate::math::Vec3;
use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut x;
    let mut y;
    let mut z;
    loop {
        x = 2.0*(rng.gen::<f64>() - 0.5);
        y = 2.0*(rng.gen::<f64>() - 0.5);
        z = 2.0*(rng.gen::<f64>() - 0.5);
        if (x*x + y*y *z*z) < 1.0
        {
            break;
        }
    }
    return Vec3{x: x, y: y, z:z};
}

pub fn reflect(in_vec: Vec3, normal: Vec3) -> Vec3 {
    return in_vec - 2.0 * normal.dot(in_vec) * normal;
}
