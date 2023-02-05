use super::*;
use std::sync::Arc;

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

//constructors
impl CheckerTexture {
    pub fn new(e: Arc<dyn Texture>, o: Arc<dyn Texture>) -> Self {
        CheckerTexture { even: e, odd: o }
    }

    pub fn from_colors(c1: Color, c2: Color) -> Self {
        CheckerTexture {
            even: Arc::new(SolidColor::from(c1)),
            odd: Arc::new(SolidColor::from(c2)),
        }
    }
}

//Checker is a Texture

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let [x, y, z] = p.as_array();
        let sines = (10.0 * x).sin() * (10.0 * y).sin() * (10.0 * z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
