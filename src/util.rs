extern crate rand;

use crate::math::{Uv, Vec3};
use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut x;
    let mut y;
    let mut z;
    loop {
        x = 2.0 * (rng.gen::<f64>() - 0.5);
        y = 2.0 * (rng.gen::<f64>() - 0.5);
        z = 2.0 * (rng.gen::<f64>() - 0.5);
        if (x * x + y * y * z * z) < 1.0 {
            break;
        }
    }
    return Vec3::new(x, y, z);
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p;
    loop {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if (p.dot(p)) < 1.0 {
            break;
        }
    }
    return p;
}

pub fn reflect(in_vec: Vec3, normal: Vec3) -> Vec3 {
    return in_vec - 2.0 * normal.dot(in_vec) * normal;
}

pub fn refract(in_vec: Vec3, normal: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = in_vec.normalized();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        return Some(ni_over_nt * (uv - normal * dt) - normal * discriminant.sqrt());
    } else {
        return None;
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r1 = r0 * r0;
    return r1 + (1.0 - r1) * (1.0 - cosine).powf(5.0);
}

pub fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum: f64 = 0.0;
    for i in 0..2 {
        let fi = i as f64;
        for j in 0..2 {
            let fj = j as f64;
            for k in 0..2 {
                let fk = k as f64;
                accum += (fi * u + (1.0 - fi) * (1.0 - u))
                    * (fj * v + (1.0 - fj) * (1.0 - v))
                    * (fk * w + (1.0 - fk) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }
    return accum;
}

pub fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;
    for i in 0..2 {
        let fi = i as f64;
        for j in 0..2 {
            let fj = j as f64;
            for k in 0..2 {
                let fk = k as f64;
                let weight_v = Vec3::new(u - fi, v - fj, w - fk);
                accum += (fi * uu + (1.0 - fi) * (1.0 - uu))
                    * (fj * vv + (1.0 - fj) * (1.0 - vv))
                    * (fk * ww + (1.0 - fk) * (1.0 - ww))
                    * weight_v.dot(c[i][j][k]);
            }
        }
    }
    return accum;
}
#[derive(Debug)]
pub struct Perlin {
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
    ranvec: Vec<Vec3>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let ranvec = Perlin::generate();
        let perm_x = Perlin::generate_perm();
        let perm_y = Perlin::generate_perm();
        let perm_z = Perlin::generate_perm();
        return Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        };
    }

    fn generate() -> Vec<Vec3> {
        let mut rng = rand::thread_rng();
        let mut ret = Vec::new();
        for _ in 0..256 {
            ret.push(Vec3::new(
                -1.0 + 2.0 * rng.gen::<f64>(),
                -1.0 + 2.0 * rng.gen::<f64>(),
                -1.0 + 2.0 * rng.gen::<f64>(),
            ));
        }
        return ret;
    }

    fn generate_perm() -> Vec<usize> {
        fn permute(p: &mut Vec<usize>) {
            let mut rng = rand::thread_rng();
            for i in (0..p.len()).rev() {
                let target = (rng.gen::<f64>() * ((i + 1) as f64)) as usize;
                let tmp = p[i];
                p[i] = p[target];
                p[target] = tmp;
            }
        }
        let mut ret = Vec::new();
        for i in 0..256 {
            ret.push(i);
        }
        permute(&mut ret);
        return ret;
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor();
        let j = p.y.floor();
        let k = p.z.floor();
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Default::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[(i + di as f64) as usize & 255]
                        ^ self.perm_y[(j + dj as f64) as usize & 255]
                        ^ self.perm_z[(k + dk as f64) as usize & 255]];
                }
            }
        }
        return perlin_interp(c, u, v, w);
    }
}

pub fn get_sphere_uv(p: &Vec3) -> Uv {
    let pp = p.normalized();
    let phi = pp.z.atan2(pp.x);
    let theta = pp.y.asin();
    let pi = std::f64::consts::PI;
    let u = 1.0 - (phi + pi) / (2.0 * pi);
    let v = (theta + pi / 2.0) / pi;
    return Uv::new(u, v);
}
