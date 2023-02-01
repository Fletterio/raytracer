use super::*;
use rand::distributions::{Distribution, Uniform};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

//constructors

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

//material

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        //ruins the lambertian distribution but keeps it safe from numerical shenaningans
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
