use core::fmt::Debug;

use crate::math::Vec3;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: &'a dyn Material,
}

pub trait Hittable: Debug {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

