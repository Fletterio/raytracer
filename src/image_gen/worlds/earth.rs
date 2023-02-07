use crate::hitable::{hitable_list::HitableList, material::lambertian::Lambertian, sphere::Sphere};
use crate::rtweekend::Point3;
use crate::texture::image::ImageTexture;
use image::ImageFormat;
use std::fs::File;
use std::sync::Arc;

pub fn globe() -> HitableList {
    let path = File::open("textures/earthmap.png").unwrap();

    let earth_texture = Arc::new(ImageTexture::new(&path, ImageFormat::Png));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    HitableList::new(vec![globe])
}
