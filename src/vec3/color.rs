use super::Color;
use std::fs::File;
use std::io::prelude::*;
use crate::rtweekend::clamp;

impl Color {
    pub fn write_color(&self, gamma : f32 ,samples : i32 ,f : &mut File) -> std::io::Result<()> {
        let mut r = self.r();
        let mut g = self.g();
        let mut b = self.b();

        //Divide color by number of samples
        let scale = 1.0 / samples as f32;
        r = (scale * r).powf(gamma);
        g = (scale * g).powf(gamma);
        b = (scale * b).powf(gamma);



        //Write

        f.write_all(format!("{} {} {}\n", (256.0 * clamp(r, 0.0, 0.999)).floor() as i32 , (256.0 * clamp(g, 0.0, 0.999)).floor() as i32, (256.0 * clamp(b, 0.0, 0.999)).floor() as i32 ).as_bytes() )?;
        Ok(())
    }
}

