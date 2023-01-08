use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vec3::Vec3;

pub trait Material {
    pub fn scatter(r_in : &Ray, rec : &HitRecord, attenuation : &Vec3, scattered : &mut Ray);
}

