use core::fmt::Debug;

use crate::hittable::HitRecord;
use crate::math::{Rgb, Uv, Vec3};
use crate::ray::Ray;

pub mod dielectric;
pub mod diffuselight;
pub mod lambertian;
pub mod metal;

pub trait Material: Debug {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)>;
    fn emitted(&self, _uv: &Uv, _p: &Vec3) -> Rgb {
        return Rgb::new(0.0, 0.0, 0.0);
    }
}
