extern crate rand;

use crate::math::{Rgb, Vec3};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::util::random_in_unit_sphere;
use super::Material;

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Rgb,
}

impl Lambertian {
    pub fn new(albedo: Rgb) -> Lambertian {
        return Lambertian {albedo: albedo};
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)> {
        let target: Vec3 = hr.p + hr.normal + random_in_unit_sphere();
        let scattered = Ray::new(hr.p, target - hr.p);
        let attenuation = self.albedo;
        return Some((scattered, attenuation));
    }
}
