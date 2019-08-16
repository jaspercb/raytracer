use super::Material;
use crate::hittable::HitRecord;
use crate::math::Rgb;
use crate::ray::Ray;
use crate::util::{random_in_unit_sphere, reflect};

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Rgb,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Rgb, f: f64) -> Metal {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        return Metal { albedo, fuzz };
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)> {
        let reflected = reflect(r.direction(), hr.normal);
        let scattered = Ray::new(hr.p, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;
        return Some((scattered, attenuation));
    }
}
