use crate::math::Vec3;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = 2.0 * oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b*b - 4.0*a*c;
        if discriminant < 0.0 {
            return None;
        }

        let temp = (-b - discriminant.sqrt()) / (2.0*a);
        if temp < t_max && temp > t_min {
            let p = r.at(temp);
            return Some(HitRecord {
                t: temp,
                p: p,
                normal: (p - self.center).normalized()
            });
        }
        let temp = (-b + discriminant.sqrt()) / (2.0*a);
        if temp < t_max && temp > t_min {
            let p = r.at(temp);
            return Some(HitRecord {
                t: temp,
                p: p,
                normal: (p - self.center).normalized()
            });
        }
        return None;
    }
}
