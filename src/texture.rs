pub mod checker;
pub mod image;
pub mod noise;

use crate::rtweekend::{Color, Point3};

/*
    Textures are just functions from [0,1]^2 to [0,1]^3, the domain being the uv-coordinates we want to sample from,
    and the image being the color of the texture at that point
*/

// Textures are Send + Sync since they're read-only for rendering threads
pub trait Texture: Sync + Send {
    // Sample the texture at (u,v). The third parameter is used to consider spatially-varying (procedural) textures
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

//------------------------------------ Simple Solid Color Texture -----------------------------

pub struct SolidColor {
    color_value: Color,
}

// Constructors
impl SolidColor {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        SolidColor {
            color_value: Color::new(red, green, blue),
        }
    }

    pub fn from(c: Color) -> Self {
        SolidColor { color_value: c }
    }
}

// Sampling a solid color always returns the same value on any coordinate
impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.color_value
    }
}
