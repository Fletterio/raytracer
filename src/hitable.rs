pub mod sphere;
pub mod hitable_list;

use crate::ray::Ray;
use crate::vec3::Vec3;



pub struct HitRecord {
    hit : bool,         //whether there was a hit
    t : f32,            //parameter on which tracing the ray hits the hitable object
    p : Vec3,           //point of impact
    normal : Vec3,      //surface normal at point of impact
}

pub trait Hitable {
    fn hit(&self, r : &Ray, t_min : f32, t_max : f32) -> HitRecord;    
}

