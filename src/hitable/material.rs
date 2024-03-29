use super::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use std::sync::Arc;

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub trait Material: Sync + Send {
    //returns whether there was scattering, and attenuation and direction of the scattered ray if so
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;

    fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
