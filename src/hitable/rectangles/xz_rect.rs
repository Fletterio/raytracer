use crate::aabb::AABB;
use crate::hitable::{material::Material, HitRecord, Hitable};
use crate::rtweekend::{Point3, Ray, Vec3};
use std::sync::Arc;

pub struct XZRect {
    material: Arc<dyn Material>,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

//constructors
impl XZRect {
    pub fn new(x0_: f32, x1_: f32, z0_: f32, z1_: f32, k_: f32, mat: Arc<dyn Material>) -> Self {
        XZRect {
            x0: x0_,
            x1: x1_,
            z0: z0_,
            z1: z1_,
            k: k_,
            material: mat,
        }
    }
}

impl Hitable for XZRect {
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        //we add padding in the Z dimension to account for the plane having no volume
        Some(AABB::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let [ox, oy, oz] = r.origin.as_array();
        let [dx, dy, dz] = r.direction.as_array();

        let t_ = (self.k - oy) / dy;
        if t_ < t_min || t_ > t_max {
            return None;
        }
        let x = ox + t_ * dx;
        let z = oz + t_ * dz;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u_ = (x - self.x0) / (self.x1 - self.x0);
        let v_ = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
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
