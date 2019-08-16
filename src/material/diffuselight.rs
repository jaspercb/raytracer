use super::Material;
use crate::hittable::HitRecord;
use crate::math::{Rgb, Uv, Vec3};
use crate::ray::Ray;
use crate::texture::Texture;

#[derive(Debug)]
pub struct DiffuseLight {
    emit: Box<Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Box<Texture>) -> Self {
        return Self { emit };
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r: &Ray, _hr: &HitRecord) -> Option<(Ray, Rgb)> {
        return None;
    }
    fn emitted(&self, uv: &Uv, p: &Vec3) -> Rgb {
        return self.emit.value(uv, p);
    }
}
