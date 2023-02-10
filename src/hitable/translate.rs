use crate::{aabb::AABB, hitable::Hitable, rtweekend::Ray, vec3::Vec3};
use std::sync::Arc;

use super::HitRecord;

pub struct Translate {
    pub object: Arc<dyn Hitable>,
    pub offset: Vec3,
}

//constructors
impl Translate {
    pub fn new(obj: Arc<dyn Hitable>, ofst: Vec3) -> Self {
        Translate {
            object: obj,
            offset: ofst,
        }
    }
}

impl Hitable for Translate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        match self.object.hit(&moved_r, t_min, t_max) {
            Some(rec) => {
                let mut rec = rec;
                rec.p += self.offset;
                rec.set_face_normal(&moved_r, &rec.normal);
                Some(rec)
            }
            None => None,
        }
    }
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        match self.object.bounding_box(time0, time1) {
            Some(output_box) => {
                let output_box =
                    AABB::new(output_box.min + self.offset, output_box.max + self.offset);
                Some(output_box)
            }
            None => None,
        }
    }
}
