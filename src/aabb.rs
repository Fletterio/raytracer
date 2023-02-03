use crate::rtweekend::{Point3, Ray};
use std::mem;
use std::simd::{f32x4, SimdFloat};

pub struct AABB {
    min: Point3,
    max: Point3,
}

//constructors

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        AABB { min: a, max: b }
    }
}

impl AABB {
    //check Ray for intersection on bounded volume
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut tm = t_min;
        let mut tM = t_max;
        for i in (0..3) {
            let invD = 1.0 / r.direction[i];
            let mut t0 = (self.min[i] - r.origin[i]) * invD;
            let mut t1 = (self.max[i] - r.origin[i]) * invD;
            if invD < 0.0 {
                mem::swap(&mut t0, &mut t1);
            };
            tm = if t0 > tm { t0 } else { tm };
            tM = if t1 < tM { t1 } else { tM };
            if tM < tm {
                return false;
            };
        }
        return true;
    }

    //Get the smallest box containing two boxes
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Point3 {
            e: box0.min.e.simd_min(box1.min.e),
        };
        let big = Point3 {
            e: box0.max.e.simd_min(box1.max.e),
        };

        AABB {
            min: small,
            max: big,
        }
    }
}
