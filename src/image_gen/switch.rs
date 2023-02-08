use super::*;

use crate::hitable::{
    hitable_list::HitableList,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    moving_sphere::MovingSphere,
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
use crate::texture::{checker::CheckerTexture, SolidColor, Texture};
use std::sync::Arc;

pub fn print() -> std::io::Result<()> {
    //output names
    let out: &str;

    //Image parameters
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 1920;
    let image_height: i32 = (image_width as f32 / aspect_ratio).round() as i32;
    let samples_per_pixel: i32 = 500;
    let max_depth: i32 = 50;

    //World setup
    let world: HitableList = HitableList::new(vec![]);

    let lookfrom: Point3;
    let lookat: Point3;
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut background = Color::new(0.0, 0.0, 0.0);

    match 0 {
        1 => {
            world = random_scene();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
            out = "balls";
        }

        2 => {
            world = spheres();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            out = "two spheres";
        }
        3 => {
            world = perlin_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            out = "perlin spheres";
        }
        4 => {
            world = globe();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(0.0, 0.0, 10.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            out = "earth"
        }
        5 | _ => {
            world = light();
            samples_per_pixel = 500;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
    };

    //Camera

    let dist_to_focus = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        UP,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    render(
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        &world,
        &background,
        &cam,
        out,
    );

    //mandatory return
    Ok(())
}
