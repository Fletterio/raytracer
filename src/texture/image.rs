use super::Texture;
use crate::perlin::Perlin;
use crate::rtweekend::{clamp, Color, Point3};
use image::{
    error::{ImageError, ImageResult},
    load, DynamicImage, ImageBuffer, ImageFormat, Rgb, RgbImage,
};
use std::fs::File;
use std::io::BufReader;

pub struct ImageTexture {
    image: Option<RgbImage>,
}

//constructor
impl ImageTexture {
    pub fn new(f: &File, format: ImageFormat) -> Self {
        let mut reader = BufReader::new(f);

        let read_image = load(reader, format);
        match read_image {
            Ok(dyn_image) => ImageTexture {
                image: Some(dyn_image.into_rgb8()),
            },
            Err(e) => {
                eprintln!("Failed to read from {:?}", f);
                ImageTexture { image: None }
            }
        }
    }
}

//IageTexture is a Texture
impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        if self.image.is_none() {
            return Color::new(0.0, 1.0, 1.0); //if there is no texture, print cyan as debugging aid
        }

        let rgb_image: &RgbImage = self.image.as_ref().unwrap();

        //We need to clamp u,v in normalized [0,1] coordinates
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0); //Flip v since image coordinates are traversed differently

        let mut i = (u * rgb_image.width() as f32).round() as u32;
        let mut j = (v * rgb_image.height() as f32).round() as u32;

        //clamp lattice coordinates to their actual range in case of overflow at the edges
        if i >= rgb_image.width() {
            i = rgb_image.width() - 1;
        };
        if j >= rgb_image.height() {
            j = rgb_image.height() - 1;
        }

        const color_scale: f32 = 1.0 / 255.0;
        let pixel = rgb_image.get_pixel(i, j);

        Color::new(
            color_scale * pixel.0[0] as f32,
            color_scale * pixel.0[1] as f32,
            color_scale * pixel.0[2] as f32,
        )
    }
}
