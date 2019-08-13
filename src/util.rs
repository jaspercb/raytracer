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

pub fn refract(in_vec: Vec3, normal: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = in_vec.normalized();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - ni_over_nt * (1.0-dt*dt);
    if discriminant > 0.0 {
        return Some(ni_over_nt*(uv - normal*dt) - normal*discriminant.sqrt());
    } else {
        return None;
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r1 = r0 * r0;
    return r1 + (1.0 - r1) * (1.0-cosine).powf(5.0);
}
