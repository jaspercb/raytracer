use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::Vec3;
use crate::ray::Ray;
use crate::util::get_sphere_uv;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = 2.0 * oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let temp = (-b - discriminant.sqrt()) / (2.0 * a);
        if temp < t_max && temp > t_min {
            let p = r.at(temp);
            let normal = (p - self.center).normalized();
            return Some(HitRecord {
                t: temp,
                p: p,
                uv: get_sphere_uv(&normal),
                normal: normal,
                mat_ptr: &*self.mat,
            });
        }
        let temp = (-b + discriminant.sqrt()) / (2.0 * a);
        if temp < t_max && temp > t_min {
            let p = r.at(temp);
            let normal = (p - self.center).normalized();
            return Some(HitRecord {
                t: temp,
                p: p,
                uv: get_sphere_uv(&normal),
                normal: normal,
                mat_ptr: &*self.mat,
            });
        }
        return None;
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        return Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ));
    }
}
