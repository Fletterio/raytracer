use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::hitable::{Hitable, sphere::Sphere, hitable_list::HitableList};
use crate::vec3::Vec3;
use crate::ray::Ray;


fn color(r : &Ray, world : &impl Hitable) -> Vec3 {
    let hit_record = world.hit(r, 0f32, f32::MAX);
    if hit_record.hit {
        0.5f32 * Vec3::new(hit_record.normal.x() + 1f32, hit_record.normal.y() + 1f32, hit_record.normal.z() + 1f32)
    }
    else {
        let unit_direction = Vec3::normalize(r.direction);
        let t = 0.5 * (unit_direction.y() + 1f32);
        (1f32 - t) * Vec3::new(1f32, 1f32, 1f32) + t * Vec3::new(0.5f32, 0.7f32, 1.0f32)
    }
}

pub fn print() {
    let path = Path::new("images/many_spheres.pmm");
    let display = path.display();
    let mut file = match File::create(&path) {  
        Err(why) => panic!("couldn't create {display} : {why}"),
        Ok(file) => file,
    };
    let nx : i32 = 200;
    let ny : i32 = 100;
    if let Err(why) = file.write_all(format!("P3\n{nx} {ny}\n255\n").as_bytes()){
        panic!("couldn't write to {display} : {why}");
    }
    let lower_left_corner = Vec3::new(-2f32, -1f32, -1f32);
    let horizontal = Vec3::new(4f32, 0f32, 0f32);
    let vertical = Vec3::new(0f32, 2f32, 0f32);
    let origin = Vec3::new(0f32, 0f32, 0f32);
    let list = vec![&Sphere::new(Vec3::new(0f32, 0f32, -1f32), 0.5f32), &Sphere::new(Vec3::new(0f32, -100.5f32, -1f32), 100f32)];

}
