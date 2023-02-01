use crate::hitable::{
    hitable_list::HitableList,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    sphere::Sphere,
    Hitable,
};
use crate::rtweekend::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdout, Write};
//use std::rc::Rc;
use crate::camera::Camera;
use crate::renderer::*;
use std::sync::Arc;

fn random_scene() -> HitableList {
    let mut world = HitableList::new(vec![]);

    let ground_material = Arc::new(Lambertian::new(Color::new(
        1.0,
        228.0 / 255.0,
        181.0 / 255.0,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f32 + 0.9 * random_double(),
                0.2,
                b as f32 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.5 {
                    //diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_between(0.5, 1.0);
                    let fuzz = random_double_between(0.0, 0.4);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    //glass
                    sphere_material = Arc::new(Dielectric::new(1.8));
                }

                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }
    let glass = Arc::new(Dielectric::new(1.7));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        glass,
    )));

    let lambert = Arc::new(Lambertian::new(Color::new(0.8, 0.1, 0.7)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        lambert,
    )));

    let metal = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        metal,
    )));

    world
}

pub fn print() -> std::io::Result<()> {
    //Image parameters
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 1920;
    let IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO).round() as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    //World setup
    let world = random_scene();

    //Camera

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        UP,
        20.0,
        16.0 / 9.0,
        aperture,
        dist_to_focus,
    );

    render(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        &world,
        &cam,
        "final",
    );

    //mandatory return
    Ok(())
}
