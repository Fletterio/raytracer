use crate::ray::Ray;
use crate::vec3::Vec3;
use super::{Material, reflect, refract};
use crate::hitable::HitRecord;
use rand::distributions::{Distribution, Uniform};

fn schlick(cosine : f32, ref_idx : f32) -> f32{
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0)* (1.0 - cosine).powi(5);
}

pub struct Dielectric {
    pub ir : f32,
}

//constructors
impl Dielectric {
    pub fn new(ri : f32) -> Dielectric { Dielectric {ref_idx : ri} }
}

impl Material for Dielectric {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Vec3, Ray)> {
        let outward_normal : Vec3;
        let reflected = reflect(&r_in.direction, &rec.normal);
        let ni_over_nt : f32;
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refracted : Vec3;
        let reflect_prob : f32;
        let cosine : f32;
        let scattered : Ray;
        if Vec3::dot(&r_in.direction, &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.len();
        }
        else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            //why is ref_idx missing here
            cosine = - self.ref_idx * Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.len();
        }
        if let Some(temp_refracted) = refract(&r_in.direction, &outward_normal, ni_over_nt) {
            refracted = temp_refracted;
            reflect_prob = schlick(cosine, self.ref_idx);
        }
        else {
            refracted = Vec3::new(0.0, 0.0, 0.0);
            reflect_prob = 1.0;
        }
        let mut rng = rand::thread_rng();
        let uniform = Uniform::from(0f32..1f32);
        if uniform.sample(&mut rng) < reflect_prob {
            scattered = Ray::new(rec.p, reflected);
        }
        else {
            scattered = Ray::new(rec.p, refracted);
        }
        return Some((attenuation, scattered));
    }
}
