use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;
use crate::ray::Ray; 
use crate::vec3::Vec3;
use crate::hitable::{Hitable, hitable_list::HitableList, sphere::Sphere, material::{Material, lambertian::Lambertian, metal::Metal}};
use crate::camera::Camera;

use rand::distributions::{Distribution, Uniform};



//depth is the number of times we scatter a ray
fn color(r : &Ray, world : &impl Hitable, depth : i32) -> Vec3 {
    if let Some(hit_record) = world.hit(r, 0.001, f32::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = hit_record.material.scatter(r, &hit_record) {
                return attenuation * color(&scattered, world, depth + 1);
            }
        }
        return Vec3::new(0f32, 0f32, 0f32);
    }
    else {
        let unit_direction = Vec3::normalize(r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}




pub fn print() -> std::io::Result<()> {
    let mut file = File::create("images/metal_spheres.ppm")?;
    //screen size
    let nx : i32 = 200;
    let ny : i32 = 100;
    //amount of antialiasing randomized samples
    let ns : i32 = 100;
    file.write_all(format!("P3\n{nx} {ny}\n255\n").as_bytes())?;
    let list : Vec<Box<dyn Hitable>> = vec!(
        Box::new(Sphere::new(
                Vec3::new(0.0, 0.0, -1.0),
                0.5,
                Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))) 
                )),
        Box::new(Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))
                )),
        Box::new(Sphere::new(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)))
                )),
        Box::new(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)))
                ))

        );

    let world = HitableList::new(list);
    let cam = Camera::default();
    //rng for antialasing
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(0f32..1f32);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns { //antialiasing
                let u = (i as f32 + uniform.sample(&mut rng)) / nx as f32;
                let v = (j as f32 + uniform.sample(&mut rng)) / ny as f32;
                let r = cam.get_ray(u,v);
                col += color(&r, &world, 0);

            }
            col /= ns as f32;
            col.gamma_correct(0.5);
            let ir = (255.0 * col.r()).round() as i32;
            let ig = (255.0 * col.g()).round() as i32;
            let ib = (255.0 * col.b()).round() as i32;
            file.write_all(format!("{ir} {ig} {ib}\n").as_bytes())?;
        }
    }
    Ok(())
}





