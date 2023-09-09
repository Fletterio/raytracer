use super::Color;
use crate::rtweekend::clamp;
use std::fs::File;
use std::io::prelude::*;
use std::simd::f32x4;

/*
    The Color class is just a Vec3 class used for shading operations
*/

impl Color {
    // The following funtion color-corrects and writes the color of a pixel to an output .ppm file
    pub fn write_color(&self, gamma: f32, samples: i32, f: &mut File) -> std::io::Result<()> {
        let self_as_array = self.as_array();

        // The color stored is actually the sum of 'samples' many samples. We average the samples to get the color for this pixel
        let scale = 1.0 / samples as f32;

        // The following code performs the same operations as this commented code below
        // r = (scale * r).powf(gamma);
        // g = (scale * g).powf(gamma);
        // b = (scale * b).powf(gamma);
        let [r, g, b]: [f32; 3] = self_as_array
            .into_iter()
            .map(|x| (scale * x).powf(gamma))
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();

        // Write to file as R8G8B8

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

    // Gamma correction
    /////////////////////---------NOTE-------------/////////////
    // This code remains unused
    #[inline]
    pub fn gamma_correct(&mut self, alpha: f32) {
        let self_as_array = self.as_array();
        self.e = f32x4::from_array(
            self_as_array
                .into_iter()
                .map(|x| x.powf(alpha))
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        );
    }
}
