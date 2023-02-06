use super::Texture;
use crate::perlin::Perlin;
use crate::rtweekend::{Color, Point3};

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f32,
}

//constructors
impl NoiseTexture {
    pub fn new(sc: f32) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

//NoiseTexture is a Texture
impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.noise.turb(&(self.scale * *p), 7) * Color::new(1.0, 1.0, 1.0)
    }
}
