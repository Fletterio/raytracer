pub mod sphere;
pub mod hitable_list;
pub mod material;

use crate::ray::Ray;
use crate::vec3::Vec3;
use material::Material;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub t : f32,                    //parameter on which tracing the ray hits the hitable object
    pub p : Vec3,                   //point of impact
    pub normal : Vec3,              //surface normal at point of impact
    pub material : Rc<dyn Material>,    //material hit
}


pub trait Hitable {
    fn hit(&self, r : &Ray, t_min : f32, t_max : f32) -> Option<HitRecord>;    
}

