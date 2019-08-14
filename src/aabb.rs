use crate::math::Vec3;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    _min: Vec3,
    _max: Vec3,
}

impl AABB {
    pub fn new(_min: Vec3, _max: Vec3) -> AABB {
        return Self {_min, _max};
    }

    pub fn min(&self) -> Vec3 {
        self._min
    }

    pub fn max(&self) -> Vec3 {
        self._max
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self._min[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self._max[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tmin = if t0 > t_min {t0} else {t_min};
            let tmax = if t1 < t_max {t1} else {t_max};
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }

    pub fn surrounding_box(&self, r: AABB) -> AABB {
        return AABB::new(Vec3::partial_min(self.min(), r.min()), Vec3::partial_max(self.max(), r.max()));
    }
}

pub fn surrounding_box(a: Option<AABB>, b: Option<AABB>) -> Option<AABB> {
    match (a, b) {
        (r, None) => r,
        (None, l) => l,
        (Some(r), Some(l)) => Some(r.surrounding_box(l))
    }
}
