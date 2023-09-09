use super::Texture;
use crate::perlin::Perlin;
use crate::rtweekend::{Color, Point3};

/*
    The NoiseTexture class stores a 3D Perlin noise texture and a scale parameter to manage the noise gradient
*/

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f32,
}

// Constructors

impl NoiseTexture {
    // The constructor only requires a scale parameter, it creates the noise itself
    pub fn new(sc: f32) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

// This type of noise is spatial, it requires no uv coordinates
// It's hardcoded to sample noise with 7 octaves. Could be modified
impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
            * 0.5
            * Color::new(1.0, 1.0, 1.0)
    }
}
