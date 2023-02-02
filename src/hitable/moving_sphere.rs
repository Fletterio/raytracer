use super::{material::Material, HitRecord, Hitable};
use crate::rtweekend::{Point3, Ray, Vec3};
use std::sync::Arc;

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Arc<dyn Material>,
}

//constructors
impl MovingSphere {
    pub fn new(c0: Point3, c1: Point3, t0: f32, t1: f32, r: f32, mat: Arc<dyn Material>) -> Self {
        MovingSphere {
            center0: c0,
            center1: c1,
            time0: t0,
            time1: t1,
            radius: r,
            material: mat,
        }
    }
}

impl MovingSphere {
    //get sphere center at a given time (must be a valid time i.e greater than time0)
    pub fn center(&self, time: f32) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.sq_len();
        let half_b = Vec3::dot(&oc, &r.direction);
        let c = oc.sq_len() - radius * radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };
        let sqrtd = f32::sqrt(discriminant);

        //find the closest hit (root) in a proper range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let impact_point = r.at(root);
        let outward_normal = (impact_point - self.center(r.time)) / self.radius;
        let mut hit_record = HitRecord {
            t: root,
            p: impact_point,
            normal: outward_normal,
            front_face: true,
            material: Arc::clone(&self.material),
        };
        hit_record.set_face_normal(r, &outward_normal);
        return Some(hit_record);
    }
}
