use crate::ray::Ray;
use crate::vec3::{Vec3, Color};
use super::HitRecord;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub trait Material : Sync + Send{
    //returns whether there was scattering, and attenuation and direction of the scattered ray if so
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Vec3, Ray)>;
}

