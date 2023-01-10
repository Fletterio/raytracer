use super::{Hitable, HitRecord, material::Material};
use crate::vec3::Vec3;
use crate::ray::Ray;
use std::rc::Rc;

pub struct Sphere {
    pub centre : Vec3,
    pub radius : f32,
    pub material : Rc <dyn Material>, 
}

//constructors

impl Sphere {
    pub fn new (c : Vec3, r : f32, mat : Rc<dyn Material>) -> Sphere {
        Sphere {centre : c, radius : r, material : mat}
    }
}

//Spheres are hitables

impl Hitable for Sphere {
   fn hit(&self, r : &Ray, t_min : f32, t_max : f32) -> Option<HitRecord> {
        let oc = r.origin - self.centre;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b*b - a * c;
        if discriminant > 0f32{
            let mut temp = (-b - f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                //rec.t = temp;
                //rec.p = r.trace(temp);
                //rec.normal = (1.0 / self.radius) * (r.trace(temp) - self.centre);
                return Some(HitRecord {t : temp, p : r.trace(temp), normal : (1.0 / self.radius) * (r.trace(temp) - self.centre), material : Rc::clone(&self.material) } );
            }
            temp = (-b + f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {t : temp, p : r.trace(temp), normal : (1.0 / self.radius) * (r.trace(temp) - self.centre), material : Rc::clone(&self.material)});
            }
        }
        return None; 
   }
}
