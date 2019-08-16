use core::fmt::Debug;

use crate::aabb::AABB;
use crate::material::Material;
use crate::math::{Uv, Vec3};
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub uv: Uv,
    pub normal: Vec3,
    pub mat_ptr: &'a dyn Material,
}

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

impl<'a, T> Hittable for &'a T
where
    T: Hittable,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        return (*self).hit(r, t_min, t_max);
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        return (*self).bounding_box(t0, t1);
    }
}
