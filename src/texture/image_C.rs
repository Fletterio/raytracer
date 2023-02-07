use super::Texture;
use crate::perlin::Perlin;
use crate::rtweekend::{clamp, Color, Point3};
use stb_image::stb_image::bindgen::{stbi_image_free, stbi_load};
use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;

const BYTES_PER_PIXEL: i32 = 3;

//required for parallelization on immutable const pointers
struct PtrWrapper {
    p: *const u8,
}

unsafe impl Sync for PtrWrapper {}
unsafe impl Send for PtrWrapper {}

pub struct ImageTexture {
    data: PtrWrapper,
    width: i32,
    height: i32,
    bytes_per_scanline: i32,
}

//constructors
impl ImageTexture {
    pub fn new(f: *const u8) -> Self {
        let components_per_pixel = BYTES_PER_PIXEL;

        unsafe {
            let mut x = 0;
            let mut y = 0;

            let d = stbi_load(
                f,
                &mut x,
                &mut y,
                &mut components_per_pixel,
                components_per_pixel,
            );

            if ptr::null() == d {
                eprintln!("ERROR: Could not load texture image file.\n");
                x = 0;
                y = 0;
            }

            ImageTexture {
                data: PtrWrapper { p: d },
                width: x,
                height: y,
                bytes_per_scanline: BYTES_PER_PIXEL * x,
            }
        }
    }
}

//destructors: must drop that nasty pointer
impl Drop for ImageTexture {
    fn drop(&mut self) {
        unsafe {
            stbi_image_free(self.data.p as *mut c_void);
        }
    }
}

//ImageTexture is a Texture
impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        if ptr::null() == self.data.p {
            return Color::new(0.0, 1.0, 1.0);
        }

        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * self.width as f32).round() as isize;
        let mut j = (v * self.height as f32).round() as isize;

        if i >= self.width as isize {
            i = self.width as isize - 1;
        }
        if j >= self.height as isize {
            j = self.height as isize - 1;
        }

        let color_scale = 1.0 / 255.0;

        unsafe {
            let pixel = self
                .data
                .p
                .offset(j * self.bytes_per_scanline as isize + i * BYTES_PER_PIXEL as isize);

            Color::new(
                color_scale * *pixel as f32,
                color_scale * *pixel.offset(1) as f32,
                color_scale * *pixel.offset(2) as f32,
            )
        }
    }
}
