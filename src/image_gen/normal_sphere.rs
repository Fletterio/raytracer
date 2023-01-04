use crate::vec3::Vec3;
use crate::ray::Ray;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn hit_sphere(centre : &Vec3, radius : f32, r : &Ray) -> f32{
    let oc = r.origin - *centre;
    let a = Vec3::dot(&r.direction, &r.direction);
    let b = 2f32 * Vec3::dot(&oc, &r.direction);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b*b - 4f32 * a * c;
    
    if discriminant < 0f32 {return -1f32;}
    else {return (-b -f32::sqrt(discriminant) ) / (2f32 * a);}
}

fn color(r : &Ray) -> Vec3 {
    let centre = Vec3::new(0f32, 0f32, -1f32);
    let t = hit_sphere(&centre, 0.5f32, r);
    if t > 0f32 {
        let n = Vec3::normalize(r.trace(t) - centre);
        return 0.5f32 * Vec3::new(n.x() + 1f32, n.y() + 1f32, n.z() + 1f32);
    }
    let unit_direction = Vec3::normalize(r.direction);
    let s = 0.5f32 * (unit_direction.y() + 1f32);
    (1f32 - s) * Vec3::new(1f32, 1f32, 1f32) + s * Vec3::new(0.5f32, 0.7f32, 1.0f32)
}

pub fn print() {
    let path = Path::new("images/normal_sphere.ppm");
    let display = path.display();
    let mut file = match File::create(&path) {
    Err(why) => panic!("could not create {} : {}", display, why),
    Ok(file) => file,
    };
    let nx : i32 = 200;
    let ny : i32 = 100;
    if let Err(why) = file.write_all(format!("P3\n{nx} {ny}\n255\n").as_bytes()) {panic!("couldn't write to {} : {}", display, why);};

    let lower_left_corner = Vec3::new(-2f32, -1f32, -1f32);
    let horizontal = Vec3::new(4f32, 0f32, 0f32);
    let vertical = Vec3::new (0f32, 2f32, 0f32);
    let origin = Vec3::new(0f32, 0f32, 0f32);
    for j in (0..ny).rev(){
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r);
            let ir = (255f32 * col.r()).round() as i32;
            let ig = (255f32 * col.g()).round() as i32;
            let ib = (255f32 * col.b()).round() as i32;

            if let Err(why) = file.write_all(format!("{ir} {ig} {ib}\n").as_bytes()){
                panic!("couldn't write to {} : {}", display, why);
            };
        }
    }


}
