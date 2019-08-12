use crate::math::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Self {origin, direction}
    }

    pub fn at(&self, t: f64) -> Vec3 {
        return self.origin + t * self.direction;
    }
}
