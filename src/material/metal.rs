use crate::math::Rgb;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::util::reflect;
use super::Material;

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Rgb,
}

impl Metal {
    pub fn new(albedo: Rgb) -> Metal {
        return Metal {albedo: albedo};
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)> {
        let scattered = Ray::new(hr.p, reflect(r.direction(), hr.normal));
        let attenuation = self.albedo;
        return Some((scattered, attenuation));
    }
}
