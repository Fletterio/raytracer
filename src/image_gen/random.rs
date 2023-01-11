use crate::rtweekend::*;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::{stdout, Write};
use crate::hitable::{Hitable, sphere::Sphere, hitable_list::HitableList, material::{Material, lambertian::Lambertian}};
use std::rc::Rc;
use crate::camera::Camera;

fn ray_color(r: &Ray, world : &impl Hitable, depth : i32) -> Color{
    //if we exceed the number of bounces
    if depth <= 0 {return Color::new(0.0, 0.0, 0.0);}

    if let Some(hit_record) = world.hit(r, DELTA, INFINITY) {
        let target = hit_record.p + Vec3::random_in_hemisphere(&hit_record.normal);
        return 0.5 * ray_color(&Ray::new(hit_record.p, target - hit_record.p), world, depth - 1);
    }
    let unit_direction = Vec3::normalize(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}


pub fn print() -> std::io::Result<()>{
    
    //Image parameters
    let ASPECT_RATIO : f32 = 16.0 / 9.0;
    let IMAGE_WIDTH : i32 = 400;
    let IMAGE_HEIGHT : i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO).round() as i32;
    const SAMPLES_PER_PIXEL : i32 = 100;
    const MAX_DEPTH : i32 = 50;
    
    //World setup
    let mut world = HitableList::new(vec![]);
    world.add(Rc::new(Sphere::new(
                Point3::new(0.0, 0.0, -1.0),
                0.5,
                Rc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)))
                )));
    world.add(Rc::new(Sphere::new(
                Point3::new(0.0, -100.5, -1.0),
                100.0,
                Rc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)))
                )));
    
    //Camera
    
    let cam = Camera::default();

    //Create folder, file, and add formatting info
    fs::create_dir_all("images/")?;

    let mut file = File::create("images/random.ppm")?;
    file.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    
    //Render

    for j in (0..IMAGE_HEIGHT).rev() {
        //show render progress on terminal
        print!("\rScanlines remaining: {}", j);
        stdout().flush();
        
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random_double()) / (IMAGE_WIDTH as f32 - 1.0);
                let v = (j as f32 + random_double()) / (IMAGE_HEIGHT as f32 - 1.0);
                let r = cam.get_ray(u,v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_color.write_color(0.5, SAMPLES_PER_PIXEL, &mut file);
        }
    }
    println!("Done!");





    //mandatory return
    Ok(())
}
