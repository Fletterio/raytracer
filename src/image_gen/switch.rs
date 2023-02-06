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
    //Image parameters
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 1920;
    let IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO).round() as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    //World setup
    let world: HitableList;

    let lookfrom: Point3;
    let lookat: Point3;
    let mut vfov = 40.0;
    let mut aperture = 0.0;

    match 0 {
        1 => {
            world = random_scene();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }

        2 | _ => {
            world = spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
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
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
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
