use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        return HittableList {hittables: Vec::new()};
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far: Option<HitRecord> = None;
        let mut dist = t_max;
        for hittable in &self.hittables
        {
            if let Some(rec) = hittable.hit(r, t_min, dist)
            {
                dist = rec.t;
                closest_so_far = Some(rec);
            }
        }
        return closest_so_far;
    }

    pub fn push(&mut self, h: Box<Hittable>) {
        self.hittables.push(h);
    }
}
