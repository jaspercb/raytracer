extern crate rand;

use crate::math::{Rgb, Vec3};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::util::random_in_unit_sphere;
use crate::texture::Texture;
use super::Material;

#[derive(Debug)]
pub struct Lambertian {
    albedo: Box<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Box<Texture>) -> Lambertian {
        return Self {albedo};
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)> {
        let target: Vec3 = hr.p + hr.normal + random_in_unit_sphere();
        let scattered = Ray::new(hr.p, target - hr.p);
        let attenuation = self.albedo.value(&hr.uv, &hr.p);
        return Some((scattered, attenuation));
    }
}
