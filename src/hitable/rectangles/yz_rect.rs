use crate::aabb::AABB;
use crate::hitable::{material::Material, HitRecord, Hitable};
use crate::rtweekend::{Point3, Ray, Vec3};
use std::sync::Arc;

pub struct YZRect {
    material: Arc<dyn Material>,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

//constructors
impl YZRect {
    pub fn new(y0_: f32, y1_: f32, z0_: f32, z1_: f32, k_: f32, mat: Arc<dyn Material>) -> Self {
        YZRect {
            y0: y0_,
            y1: y1_,
            z0: z0_,
            z1: z1_,
            k: k_,
            material: mat,
        }
    }
}

impl Hitable for YZRect {
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        //we add padding in the Z dimension to account for the plane having no volume
        Some(AABB::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let [ox, oy, oz] = r.origin.as_array();
        let [dx, dy, dz] = r.direction.as_array();

        let t_ = (self.k - ox) / dx;
        if t_ < t_min || t_ > t_max {
            return None;
        }
        let y = oy + t_ * dy;
        let z = oz + t_ * dz;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u_ = (y - self.y0) / (self.y1 - self.y0);
        let v_ = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
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
