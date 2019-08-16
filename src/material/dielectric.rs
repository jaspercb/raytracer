extern crate rand;

use rand::Rng;

use super::Material;
use crate::hittable::HitRecord;
use crate::math::Rgb;
use crate::ray::Ray;
use crate::util::{reflect, refract, schlick};

#[derive(Debug, Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        return Dielectric { ref_idx };
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)> {
        let (outward_normal, ni_over_nt, cosine) = if r.direction().dot(hr.normal) > 0.0 {
            (
                -hr.normal,
                self.ref_idx,
                self.ref_idx * r.direction().dot(hr.normal) / r.direction().magnitude(),
            )
        } else {
            (
                hr.normal,
                1.0 / self.ref_idx,
                -r.direction().dot(hr.normal) / r.direction().magnitude(),
            )
        };
        let attenuation = Rgb::one();
        let reflect_prob: f64;
        let refracted = refract(r.direction(), outward_normal, ni_over_nt);
        let mut rng = rand::thread_rng();
        match refracted {
            Some(refracted_vec) => {
                reflect_prob = schlick(cosine, self.ref_idx);
                if rng.gen::<f64>() < reflect_prob {
                    return Some((
                        Ray::new(hr.p, reflect(r.direction(), hr.normal)),
                        attenuation,
                    ));
                } else {
                    return Some((Ray::new(hr.p, refracted_vec), attenuation));
                }
            }
            _ => {
                return Some((
                    Ray::new(hr.p, reflect(r.direction(), hr.normal)),
                    attenuation,
                ));
            }
        }
    }
}
