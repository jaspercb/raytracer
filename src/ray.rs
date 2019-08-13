use crate::math::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        let normalized_dir: Vec3 = direction.normalized();
        Self {origin, direction: normalized_dir}
    }

    pub fn at(&self, t: f64) -> Vec3 {
        return self.origin + t * self.direction;
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
