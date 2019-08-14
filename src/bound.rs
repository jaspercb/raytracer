use crate::aabb::AABB;

pub trait Bound {
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}
