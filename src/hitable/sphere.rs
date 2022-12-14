use super::{Hitable, HitRecord, material::Material};
use crate::rtweekend::{Vec3, Point3, Ray};
use std::rc::Rc;

pub struct Sphere {
    pub centre : Point3,
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
        let a = r.direction.sq_len();
        let half_b = Vec3::dot(&oc, &r.direction);
        let c = oc.sq_len() - self.radius * self.radius;
        let discriminant = half_b*half_b - a * c;
        if discriminant < 0f32 {return None}

        let sqrtd = f32::sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {return None}
        }
        let impact_point = r.at(root);
        let outward_normal = (1.0 / self.radius) * (impact_point - self.centre);
        let mut hit_record = HitRecord {t : root, p : impact_point, normal : outward_normal, front_face : true ,material : Rc::clone(&self.material)};
        hit_record.set_face_normal(r, &outward_normal);
        return Some(hit_record);
   } 
}
