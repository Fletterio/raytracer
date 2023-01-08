pub mod sphere;
pub mod hitable_list;

use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HitRecord <'a> {
    pub t : f32,                    //parameter on which tracing the ray hits the hitable object
    pub p : Vec3,                   //point of impact
    pub normal : Vec3,              //surface normal at point of impact
    pub material : &'a Material,    //material hit
}


pub trait Hitable {
    fn hit(&self, r : &Ray, t_min : f32, t_max : f32, rec : &mut HitRecord) -> bool;    
}

