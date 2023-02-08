use crate::hitable::{material::Material, HitRecord};
use crate::rtweekend::{Color, Ray, Vec3};
use crate::texture::Texture;
use std::sync::Arc;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Arc<dyn Texture>,
    pub fuzz: f32,
}

//constructors

impl Metal {
    pub fn new(a: Arc<dyn Texture>, f: f32) -> Metal {
        let fuzziness = f.min(1.0);
        Metal {
            albedo: a,
            fuzz: fuzziness,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&Vec3::normalize(r_in.direction), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.time,
        );
        if Vec3::dot(&scattered.direction, &rec.normal) > 0f32 {
            return Some((self.albedo.value(rec.u, rec.v, &rec.p), scattered));
        } else {
            return None;
        }
    }
}
