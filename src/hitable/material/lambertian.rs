use super::*;
use crate::texture::*;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

//constructors

impl Lambertian {
    pub fn new(a: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo: a }
    }
    pub fn from_color(c: Color) -> Self {
        Lambertian {
            albedo: Arc::new(SolidColor::from(c)),
        }
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

        let scattered = Ray::new(rec.p, scatter_direction, r_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }
}
