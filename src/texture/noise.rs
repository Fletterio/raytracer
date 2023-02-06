use super::Texture;
use crate::perlin::Perlin;
use crate::rtweekend::{Color, Point3};

pub struct NoiseTexture {
    pub noise: Perlin,
}

//constructors
impl NoiseTexture {
    pub fn new() -> Self {
        NoiseTexture {
            noise: Perlin::new(),
        }
    }
}

//NoiseTexture is a Texture
impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.noise.noise(p) * Color::new(1.0, 1.0, 1.0)
    }
}
