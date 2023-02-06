use crate::hitable::{
    hitable_list::HitableList,
    material::{lambertian::Lambertian, Material},
    sphere::Sphere,
    Hitable,
};
use crate::rtweekend::Point3;
use crate::texture::{noise::NoiseTexture, Texture};
use std::sync::Arc;

pub fn perlin_spheres() -> HitableList {
    let objects = HitableList::new(Vec::new());

    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new());
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
