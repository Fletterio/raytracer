use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn print() {
    let path = Path::new("simple.ppm");
    let display = path.display();
    let mut file = match File::create(&path){
        Err(why) => panic!("no se pudo crear {}: {}", display, why),
        Ok(file) => file,
    };
    let nx : i32 = 200;
    let ny : i32 = 100;
    match file.write_all(format!("P3\n{nx} {ny}\n255\n").as_bytes()){
        Err(why) => panic!("no se pudo escribir a {display}: {why}"),
        _ => (),
    }
    for j in (0..ny).rev() {
        for i in 0..nx{
            let r : f32 = (i as f32) / (nx as f32);
            let g : f32 = (j as f32) / (ny as f32);
            let b : f32 = 0.2f32;
            let ir : i32 = (255.99 * r).round() as i32;
            let ig : i32 = (255.99 * g).round() as i32;
            let ib : i32 = (255.99 * b).round() as i32;
            match file.write_all(format!("{ir} {ig} {ib}\n").as_bytes()){
                Err(why) => panic!("no se pudo escribir a {display}: {why}"),
                _ => (),
            }
        }
    }
}


