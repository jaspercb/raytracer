use crate::math::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom-lookat).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);
        let lower_left_corner = lookfrom - half_width*u - half_height*v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;
        return Camera {origin: lookfrom, lower_left_corner, horizontal, vertical};
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin);
    }
}
