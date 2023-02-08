pub mod checker;
pub mod image;
pub mod noise;

use crate::rtweekend::{Color, Point3};

pub trait Texture: Sync + Send {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

//Solid_Color constructors
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

//a Solid_Color is a texture
impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.color_value
    }
}
