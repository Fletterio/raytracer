pub mod hitable_list;
pub mod material;
pub mod sphere;

use crate::rtweekend::{Point3, Ray, Vec3};
use material::Material;
use std::marker::Sync;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,       //parameter on which tracing the ray hits the hitable object
    pub p: Point3,    //point of impact
    pub normal: Vec3, //surface normal at point of impact
    pub front_face: bool,
    pub material: Arc<dyn Material>, //material hit
}

//determine which face we hit
impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hitable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
