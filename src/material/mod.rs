use core::fmt::Debug;

use crate::math::{Rgb, Uv, Vec3};
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod diffuselight;

pub trait Material: Debug {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)>;
    fn emitted(&self, _uv: &Uv, _p: &Vec3) -> Rgb {
        return Rgb::new(0.0, 0.0, 0.0);
    }
}
