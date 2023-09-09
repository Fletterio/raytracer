use crate::camera::Camera;
use crate::hitable::Hitable;
use crate::rtweekend::*;
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::io::{stdout, Write};

/*
    This module handles all of the rendering
*/

// -------------------------- AUX functions --------------------------

// The following function computes the color for a ray recursively: At each impact of the ray with an object it adds some color
// and scatters the ray (where possible) to further add color, with some attenuation depending on the material
pub fn ray_color(r: &Ray, background: &Color, world: &impl Hitable, depth: i32) -> Color {
    //if we exceed the number of bounces, don't add any Color
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(r, DELTA, INFINITY) {
        // If the ray hits something...
        Some(rec) => {
            let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
            //if the material allows for scattering, scatter the ray. Otherwise just return the emitted color
            match rec.material.scatter(r, &rec) {
                Some((attenuation, scattered)) => {
                    emitted + attenuation * ray_color(&scattered, &background, world, depth - 1)
                }
                None => emitted,
            }
        }
        // If the ray doesn't hit anything, return a background color
        None => *background,
    }
}

// ---------------------------------------- RENDERING -------------------------------------

pub fn render(
    width: i32,           // viewport width
    height: i32,          // viewport height
    samples: i32,         // number of samples per pixel
    depth: i32,           // how many times a ray is allowed to bounce
    world: &impl Hitable, // description of the world scene
    background: &Color,   // background color if a ray doesn't hit anything
    cam: &Camera,         // camera used to render the scene
    out: &str,            // name for the output .ppm file
) {
    // First we create a folder to store output images, the output file, and add formatting info
    fs::create_dir_all("images/").unwrap();

    let path = format!("{}{}{}", "images/", out, ".ppm");

    let mut file = File::create(path).unwrap();
    file.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())
        .unwrap();

    // For each horizontal line
    for j in (0..height).rev() {
        // Show render progress on terminal, indicating how many horizontal lines have been rendered
        print!("\rScanlines remaining: {}", j);
        stdout().flush();

        for i in 0..width {
            // The following code is a bit murky. Basically we launch a concurrent thread for each sample for the pixel (i,j) using rayon.
            // On each thread we create a slightly fuzzified ray emanating from the camera and hitting the viewport at pixel (i,j)
            // (could also use a regular sampling scheme) and compute that ray's color.
            // Then we accumulate the color obtained on each thread and use the helper function 'write_color' to write the color for the pixel
            // to the output file.
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
