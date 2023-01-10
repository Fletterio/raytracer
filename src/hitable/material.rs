use crate::ray::Ray;
use crate::vec3::Vec3;
use super::HitRecord;

pub mod lambertian;
pub mod metal;

pub trait Material {
    //returns whether there was scattering, and attenuation and direction of the scattered ray if so
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Vec3, Ray)>;
}
