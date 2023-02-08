use crate::hitable::{hitable_list::HitableList, material::lambertian::Lambertian, sphere::Sphere};
use crate::rtweekend::{Color, Point3};
use crate::texture::{checker::CheckerTexture, Texture};
use std::sync::Arc;

pub fn spheres() -> HitableList {
    let mut objects = HitableList::new(Vec::new());

    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(Arc::clone(&checker))),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(Arc::clone(&checker))),
    )));

    objects
}
