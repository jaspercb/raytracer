use core::fmt::Debug;

use crate::math::Rgb;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub trait Material: Debug {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)>;
}
