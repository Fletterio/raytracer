use crate::camera::Camera;
use crate::hitable::{material::Material, Hitable};
use crate::rtweekend::*;
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdout, Write};
use std::sync::Arc;

pub fn ray_color(r: &Ray, background: &Color, world: &impl Hitable, depth: i32) -> Color {
    //if we exceed the number of bounces, don't add any Color
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(r, DELTA, INFINITY) {
        Some(rec) => {
            let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
            match rec.material.scatter(r, &rec) {
                Some((attenuation, scattered)) => {
                    emitted + attenuation * ray_color(&scattered, &background, world, depth - 1)
                }
                None => emitted,
            }
        }
        None => *background,
    }

    /*if let Some(hit_record) = world.hit(r, DELTA, INFINITY) {
        if let Some((attenuation, scattered)) = hit_record.material.scatter(r, &hit_record) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = Vec3::normalize(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);*/
}

pub fn render(
    width: i32,
    height: i32,
    samples: i32,
    depth: i32,
    world: &impl Hitable,
    background: &Color,
    cam: &Camera,
    out: &str,
) {
    //Create folder, file, and add formatting info
    fs::create_dir_all("images/").unwrap();

    let path = format!("{}{}{}", "images/", out, ".ppm");

    let mut file = File::create(path).unwrap();
    file.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())
        .unwrap();

    for j in (0..height).rev() {
        //show render progress on terminal
        print!("\rScanlines remaining: {}", j);
        stdout().flush();

        for i in 0..width {
            let pixel_color: Color = (0..samples)
                .into_par_iter()
                .map(|_| {
                    let u = (i as f32 + random_double()) / (width as f32 - 1.0);
                    let v = (j as f32 + random_double()) / (height as f32 - 1.0);
                    let r = cam.get_ray(u, v);
                    ray_color(&r, background, world, depth)
                })
                .reduce(|| Color::new(0.0, 0.0, 0.0), |acc, elem| acc + elem);

            pixel_color.write_color(0.5, samples, &mut file);
        }
    }
    println!("Done!");
}
