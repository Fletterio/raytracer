use super::{Hitable, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;


pub struct Sphere {
    centre : Vec3,
    radius : f32,
}

//constructors

impl Sphere {
    pub fn new(c : Vec3, r : f32) -> Sphere {
        Sphere {centre : c, radius : r}
    }
}

//Spheres are hitables

impl Hitable for Sphere {
   fn hit(&self, r : &Ray, t_min : f32, t_max : f32) -> HitRecord {
        let oc = r.origin - self.centre;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b*b - a * c;
        if discriminant > 0f32{
            let mut temp = (-b - f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                return HitRecord {hit : true, t : temp, p : r.trace(temp), normal :(1f32 / self.radius) * (r.trace(temp) - self.centre)};
            }
            temp = (-b + f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                return HitRecord {hit : true, t : temp, p : r.trace(temp), normal : (1f32 / self.radius) * (r.trace(temp) - self.centre)};
            }
        }
        return HitRecord {hit : false, t : 0f32, p: r.origin, normal : r.origin};
    }
}
