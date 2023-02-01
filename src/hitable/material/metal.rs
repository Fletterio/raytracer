use crate::hitable::{material::Material, HitRecord};
use crate::rtweekend::{Ray, Vec3};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

//constructors

impl Metal {
    pub fn new(a: Vec3, f: f32) -> Metal {
        let fuzziness = f.min(1.0);
        Metal {
            albedo: a,
            fuzz: fuzziness,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::reflect(&Vec3::normalize(r_in.direction), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if Vec3::dot(&scattered.direction, &rec.normal) > 0f32 {
            return Some((self.albedo, scattered));
        } else {
            return None;
        }
    }
}
