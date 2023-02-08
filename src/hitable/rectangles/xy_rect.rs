use crate::aabb::AABB;
use crate::hitable::{material::Material, HitRecord, Hitable};
use crate::rtweekend::{Point3, Ray, Vec3};
use std::sync::Arc;

pub struct XYRect {
    material: Arc<dyn Material>,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

//constructors
impl XYRect {
    pub fn new(x0_: f32, x1_: f32, y0_: f32, y1_: f32, k_: f32, mat: Arc<dyn Material>) -> Self {
        XYRect {
            x0: x0_,
            x1: x1_,
            y0: y0_,
            y1: y1_,
            k: k_,
            material: mat,
        }
    }
}

impl Hitable for XYRect {
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        //we add padding in the Z dimension to account for the plane having no volume
        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let [ox, oy, oz] = r.origin.as_array();
        let [dx, dy, dz] = r.direction.as_array();

        let t_ = (self.k - oz) / dz;
        if t_ < t_min || t_ > t_max {
            return None;
        }
        let x = ox + t_ * dx;
        let y = oy + t_ * dy;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u_ = (x - self.x0) / (self.x1 - self.x0);
        let v_ = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let p_ = r.at(t_);
        let mut rec = HitRecord {
            t: t_,
            p: p_,
            normal: outward_normal,
            front_face: true,
            material: Arc::clone(&self.material),
            u: u_,
            v: v_,
        };
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }
}
