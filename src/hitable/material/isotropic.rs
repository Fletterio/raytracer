use std::sync::Arc;

use crate::{
    hitable::HitRecord,
    rtweekend::Ray,
    texture::{SolidColor, Texture},
    vec3::{Color, Vec3},
};

use super::Material;

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

//constructors
impl Isotropic {
    pub fn new(a: Arc<dyn Texture>) -> Self {
        Isotropic { albedo: a }
    }

    pub fn from_color(c: Color) -> Self {
        Isotropic {
            albedo: Arc::new(SolidColor::from(c)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }
}
