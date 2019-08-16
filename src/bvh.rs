extern crate rand;

use rand::Rng;

use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Debug)]
pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bound: AABB,
}

impl BvhNode {
    pub fn construct(
        hittables: &mut Vec<Box<dyn Hittable>>,
        t0: f64,
        t1: f64,
    ) -> Box<dyn Hittable> {
        let mut rng = rand::thread_rng();

        if hittables.len() == 1 {
            return hittables.remove(0);
        }
        if hittables.len() == 2 {
            let left = hittables.remove(1);
            let right = hittables.remove(0);
            let bound =
                surrounding_box(right.bounding_box(t0, t1), left.bounding_box(t0, t1)).unwrap();
            return Box::new(BvhNode { right, left, bound });
        }

        let axis = (3.0 * rng.gen::<f32>()) as usize;

        hittables.sort_by(
            |a, b| match (a.bounding_box(t0, t1), b.bounding_box(t0, t1)) {
                (Some(a), Some(b)) => a.min()[axis].partial_cmp(&b.min()[axis]).unwrap(),
                _ => panic!("no bounding box in BvhNode constructor"),
            },
        );
        let n = hittables.len() / 2 as usize;
        let mut left_nodes: Vec<Box<dyn Hittable>> = Vec::new();
        let mut right_nodes: Vec<Box<dyn Hittable>> = Vec::new();

        for i in (n..hittables.len()).rev() {
            right_nodes.push(hittables.remove(i));
        }
        for i in (0..n).rev() {
            left_nodes.push(hittables.remove(i));
        }

        let right = BvhNode::construct(&mut left_nodes, t0, t1);
        let left = BvhNode::construct(&mut right_nodes, t0, t1);
        let bound = surrounding_box(right.bounding_box(t0, t1), left.bounding_box(t0, t1)).unwrap();
        return Box::new(BvhNode { right, left, bound });
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bound.hit(r, t_min, t_max) {
            return match (
                self.left.hit(r, t_min, t_max),
                self.right.hit(r, t_min, t_max),
            ) {
                (None, right) => right,
                (left, None) => left,
                (Some(a), Some(b)) => Some(if a.t < b.t { a } else { b }),
            };
        }
        return None;
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        return Some(self.bound);
    }
}
