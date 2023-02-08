use crate::{
    hitable::{
        hitable_list::HitableList,
        material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
        rectangles::XYRect,
        sphere::Sphere,
    },
    rtweekend::{Color, Point3},
    texture::{image::ImageTexture, noise::NoiseTexture, Texture},
};
use image::ImageFormat;
use std::fs::File;
use std::sync::Arc;

pub fn light() -> HitableList {
    let mut objects = HitableList::new(vec![]);

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

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    let redlight = Arc::new(DiffuseLight::from_color(Color::new(8.0, 0.0, 0.0)));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 20.0, 0.0),
        10.0,
        redlight,
    )));

    let moon_path = File::open("textures/moon.png").unwrap();

    let moon = Arc::new(ImageTexture::new(&moon_path, ImageFormat::Png));
    objects.add(Arc::new(Sphere::new(
        Point3::new(3.0, 2.3, 3.0),
        1.5,
        Arc::new(Lambertian::new(moon)),
    )));

    objects
}
