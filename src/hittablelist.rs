use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct HittableList {
    hittables: Vec<Box<Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        return HittableList {hittables: Vec::new()};
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far: Option<HitRecord> = None;
        for hittable in &self.hittables
        {
            match (closest_so_far, hittable.hit(r, t_min, t_max)) {
                (Some(c), Some(r)) => {
                    if r.t < c.t {
                        closest_so_far = Some(r);
                    }
                },
                (None, Some(r)) => {
                    closest_so_far = Some(r);
                },
                _ => (),
            }
        }
        return closest_so_far;
    }

    pub fn push(&mut self, h: Box<Hittable>) {
        self.hittables.push(h);
    }
}
