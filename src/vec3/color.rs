use super::Color;
use crate::rtweekend::clamp;
use std::fs::File;
use std::io::prelude::*;

impl Color {
    pub fn write_color(&self, gamma: f32, samples: i32, f: &mut File) -> std::io::Result<()> {
        let self_as_array = self.as_array();

        //Divide color by number of samples
        let scale = 1.0 / samples as f32;
        //r = (scale * r).powf(gamma);
        //g = (scale * g).powf(gamma);
        //b = (scale * b).powf(gamma);
        let [r, g, b]: [f32; 3] = self_as_array
            .into_iter()
            .map(|x| (scale * x).powf(gamma))
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();

        //Write

        f.write_all(
            format!(
                "{} {} {}\n",
                (256.0 * clamp(r, 0.0, 0.999)).floor() as i32,
                (256.0 * clamp(g, 0.0, 0.999)).floor() as i32,
                (256.0 * clamp(b, 0.0, 0.999)).floor() as i32
            )
            .as_bytes(),
        )?;
        Ok(())
    }
}
