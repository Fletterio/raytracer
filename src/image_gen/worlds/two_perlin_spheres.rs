use crate::hitable::{hitable_list::HitableList, material::lambertian::Lambertian, sphere::Sphere};
use crate::rtweekend::Point3;
use crate::texture::{noise::NoiseTexture, Texture};
use std::sync::Arc;

pub fn perlin_spheres() -> HitableList {
    let mut objects = HitableList::new(Vec::new());

    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Arc::clone(&pertext))),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(Arc::clone(&pertext))),
    )));

    objects
}
