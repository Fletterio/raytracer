use std::simd::{f32x4, SimdFloat};
use std::sync::Arc;

use crate::hitable::HitRecord;
use crate::rtweekend::Ray;
use crate::{aabb::AABB, hitable::Hitable, rtweekend::deg_to_rad, vec3::Point3};

pub struct RotateX {
    object: Arc<dyn Hitable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

//constructors
impl RotateX {
    pub fn new(
        obj: Arc<dyn Hitable>,
        angle: f32, //expressed in degrees
    ) -> Self {
        let radians = deg_to_rad(angle);
        let sin_t = radians.sin();
        let cos_t = radians.cos();
        let bbox_ = obj.bounding_box(0.0, 1.0);
        if bbox_.is_none() {
            return RotateX {
                object: obj,
                sin_theta: sin_t,
                cos_theta: cos_t,
                bbox: None,
            };
        }
        let bbox_ = bbox_.unwrap();

        let mut min = f32x4::splat(f32::INFINITY);
        let mut max = f32x4::splat(-f32::INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let ijk = f32x4::from_array([i as f32, j as f32, k as f32, 0.0]);
                    let comp_ijk = f32x4::splat(1.0) - ijk;
                    let [x, y, z, _w] = *(ijk * bbox_.max.e + comp_ijk * bbox_.min.e).as_array();
                    let newy = cos_t * y - sin_t * z;
                    let newz = sin_t * y + cos_t * z;

                    let tester = f32x4::from_array([x, newy, newz, 0.0]);

                    min = min.simd_min(tester);
                    max = max.simd_max(tester);
                }
            }
        }
        RotateX {
            object: obj,
            sin_theta: sin_t,
            cos_theta: cos_t,
            bbox: Some(AABB::new(Point3 { e: min }, Point3 { e: max })),
        }
    }
}

impl Hitable for RotateX {
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.bbox
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let r_origin = r.origin.as_array();
        let r_direction = r.direction.as_array();

        let mut origin = r_origin;
        let mut direction = r_direction;

        origin[1] = self.cos_theta * r_origin[1] + self.sin_theta * r_origin[2];
        origin[2] = -self.sin_theta * r_origin[1] + self.cos_theta * r_origin[2];

        direction[1] = self.cos_theta * r_direction[1] + self.sin_theta * r_direction[2];
        direction[2] = -self.sin_theta * r_direction[1] + self.cos_theta * r_direction[2];

        let rotated_r = Ray::new(
            Point3::from_array(origin),
            Point3::from_array(direction),
            r.time,
        );

        let rec = self.object.hit(&rotated_r, t_min, t_max);
        if rec.is_none() {
            return None;
        }
        let mut rec = rec.unwrap();

        let rec_p = rec.p.as_array();
        let rec_normal = rec.normal.as_array();

        let mut p_ = rec_p;
        let mut normal = rec_normal;

        p_[1] = self.cos_theta * rec_p[1] - self.sin_theta * rec_p[2];
        p_[2] = self.sin_theta * rec_p[1] + self.cos_theta * rec_p[2];

        normal[1] = self.cos_theta * rec_normal[1] - self.sin_theta * rec_normal[2];
        normal[2] = self.sin_theta * rec_normal[1] + self.cos_theta * rec_normal[2];

        rec.p = Point3::from_array(p_);
        rec.set_face_normal(&rotated_r, &Point3::from_array(normal));

        Some(rec)
    }
}
