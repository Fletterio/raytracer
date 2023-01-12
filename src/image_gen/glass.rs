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


pub fn print() -> std::io::Result<()>{
    
    //Image parameters
    const ASPECT_RATIO : f32 = 16.0 / 9.0;
    const IMAGE_WIDTH : i32 = 1920;
    let IMAGE_HEIGHT : i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO).round() as i32;
    const SAMPLES_PER_PIXEL : i32 = 100;
    const MAX_DEPTH : i32 = 50;
    
    //some materials here
    let ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let copper = Rc::new(Metal::new(Color::new(1.0, 94.0/255.0, 5.0/255.0), 0.7));
    let velvet = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let silver = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    let steel = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.15));
    let gold = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    let bluish = Rc::new(Metal::new(Color::new(0.3, 0.3, 0.9), 0.1));
    let reddish = Rc::new(Metal::new(Color::new(1.0, 0.2, 0.2), 0.00));
    
    let glass = Rc::new(Dielectric::new(1.7));

    //World setup
    let mut world = HitableList::new(vec![]);
   
    world.add(Rc::new(Sphere::new(
                Point3::new(0.0, -100.5, -1.0),
                100.0,
                copper
                )));
    world.add(Rc::new(Sphere::new(
                Point3::new(-1.0, 0.0, -0.8),
                0.5,
                Rc::clone(&glass) as Rc<dyn Material>
                )));
    world.add(Rc::new(Sphere::new(
                Point3::new(-1.0, 0.0, -0.8),
                -0.45,
                glass
                )));
    world.add(Rc::new(Sphere::new(
                Point3::new(-0.5, 0.0, 0.3),
                0.5,
                reddish
                )));
    world.add(Rc::new(Sphere::new(
                Point3::new(0.5, -0.2, -0.7),
                0.3,
                gold
                )));
world.add(Rc::new(Sphere::new(
                Point3::new(0.0, 0.4, -1.2),
                0.7,
                silver
                )));
    //Camera
   
    let lookfrom = Point3::new(-2.0, 2.0, 1.0);
    let lookat = Point3::new(0.0, 0.0, -0.5);
    let dist_to_focus = (lookfrom - lookat).len();
    let aperture = 0.1;

    let cam = Camera::new(lookfrom, lookat, UP ,40.0, 16.0 / 9.0, aperture, dist_to_focus);

    //Create folder, file, and add formatting info
    fs::create_dir_all("images/")?;

    let mut file = File::create("images/glass.ppm")?;
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
