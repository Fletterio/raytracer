use super::*;
use rand::distributions::{Distribution, Uniform};

fn random_in_unit_sphere() -> Vec3 {
    let uniform = Uniform::from(-1f32..1f32);
    let mut rng = rand::thread_rng();
    let mut p = Vec3::new(1f32, 1f32, 1f32);
    while Vec3::dot(&p,&p) >= 1.0 {
        p = Vec3::new(uniform.sample(&mut rng), uniform.sample(&mut rng), uniform.sample(&mut rng));
    }
    return p;
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Lambertian {
    pub albedo : Color, 
}

//constructors

impl Lambertian {
    pub fn new(a : Color) -> Lambertian {
        Lambertian {albedo : a}
    }
}

//material


impl Material for Lambertian {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Color, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        return Some((self.albedo, scattered));
    } 
}
