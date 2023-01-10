use crate::vec3::Vec3;
use crate::hitable::{HitRecord, material::Material};
use crate::ray::Ray;
use super::reflect;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Metal {
    pub albedo : Vec3,
}

//constructors

impl Metal {
    pub fn new(a : Vec3) -> Metal{
        Metal {albedo : a}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&Vec3::normalize(r_in.direction), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        if Vec3::dot(&scattered.direction, &rec.normal) > 0f32 {
            return Some((self.albedo, scattered));    
        }
        else {
            return None;    
        }
        
    }
} 
