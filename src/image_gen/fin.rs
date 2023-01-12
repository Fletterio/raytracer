use crate::rtweekend::*;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::{stdout, Write};
use crate::hitable::{Hitable, sphere::Sphere, hitable_list::HitableList, material::{Material, lambertian::Lambertian, metal::Metal, dielectric::Dielectric}};
use std::rc::Rc;
use crate::camera::Camera;

fn ray_color(r: &Ray, world : &impl Hitable, depth : i32) -> Color{
    //if we exceed the number of bounces
    if depth <= 0 {return Color::new(0.0, 0.0, 0.0);}

    if let Some(hit_record) = world.hit(r, DELTA, INFINITY) {
        if let Some((attenuation, scattered)) = hit_record.material.scatter(r, &hit_record) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = Vec3::normalize(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn random_scene() -> HitableList {
    let mut world = HitableList::new(vec![]);

    let ground_material = Rc::new(Lambertian::new(Color::new(1.0, 228.0 / 255.0, 181.0 / 255.0)));
    world.add(Rc::new(Sphere::new(
                Point3::new(0.0, -1000.0, 0.0),
                1000.0,
                ground_material
                )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(a as f32 + 0.9 * random_double(), 0.2, b as f32 + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material : Rc<dyn Material>;

                if choose_mat < 0.5 {
                    //diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                }
                else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_between(0.5, 1.0);
                    let fuzz = random_double_between(0.0, 0.4);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                }
                else {
                    //glass
                    sphere_material = Rc::new(Dielectric::new(1.8));
                }
                
                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }
    let glass = Rc::new(Dielectric::new(1.7));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, glass)));

    let lambert = Rc::new(Lambertian::new(Color::new(0.8, 0.1, 0.7)));
    world.add(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, lambert)));

    let metal = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    world.add(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, metal)));

    world
    
}

pub fn print() -> std::io::Result<()>{
    
    //Image parameters
    const ASPECT_RATIO : f32 = 16.0 / 9.0;
    const IMAGE_WIDTH : i32 = 1920;
    let IMAGE_HEIGHT : i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO).round() as i32;
    const SAMPLES_PER_PIXEL : i32 = 500;
    const MAX_DEPTH : i32 = 50;
    
    //World setup
    let world = random_scene();

    //Camera
   
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(lookfrom, lookat, UP ,20.0, 16.0 / 9.0, aperture, dist_to_focus);

    //Create folder, file, and add formatting info
    fs::create_dir_all("images/")?;

    let mut file = File::create("images/final.ppm")?;
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
