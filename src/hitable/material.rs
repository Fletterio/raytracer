use crate::ray::Ray;
use crate::vec3::{Vec3, Color};
use super::HitRecord;

pub mod lambertian;
pub mod metal;

pub trait Material {
    //returns whether there was scattering, and attenuation and direction of the scattered ray if so
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Vec3, Ray)>;
}

fn reflect(v : &Vec3, n : &Vec3) -> Vec3 {
    *v - 2f32 * Vec3::dot(v,n) * *n
}


//Returns whether there is refraction and in which direction. ni_over_nt is the refractive
//index ratio.
//UNENFORCED: Normals should be unit vectors
fn refract(v : &Vec3, n : &Vec3, etai_over_etat : f32) -> Vec3 {
    let uv = Vec3::normalize(*v);
    let cos_theta = 1f32.min(Vec3::dot(&(-uv), n));
    let r_out_perp = etai_over_etat * (uv + cos_theta * *n);
    let r_out_parallel = - f32::sqrt( (1.0 - r_out_perp.sq_len()).abs() ) * *n;
    return r_out_perp + r_out_parallel;
}
