use super::Material;
use crate::hitable::HitRecord;
use crate::rtweekend::{random_double, Color, Ray, Vec3};

pub struct Dielectric {
    pub ir: f32, //index of refraction
}

//constructors
impl Dielectric {
    pub fn new(ri: f32) -> Dielectric {
        Dielectric { ir: ri }
    }
}

#[inline]
fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    //Schlick's approximation for reflectance
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::normalize(r_in.direction);
        let cos_theta = Vec3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        let scattered = Ray::new(rec.p, direction, r_in.time);
        return Some((attenuation, scattered));
    }
}
