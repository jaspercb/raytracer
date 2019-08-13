use crate::math::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let lower_left_corner = Vec3 {x: -half_width, y: -half_height, z: -1.0};
        let horizontal = Vec3 {x: 2.0 * half_width, y: 0.0, z: 0.0};
        let vertical = Vec3 {x: 0.0, y: 2.0 * half_height, z: 0.0};
        let origin = Vec3 {x: 0.0, y: 0.0, z: 0.0};
        return Camera {origin: origin, lower_left_corner: lower_left_corner, horizontal: horizontal, vertical: vertical};
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical);
    }
}
