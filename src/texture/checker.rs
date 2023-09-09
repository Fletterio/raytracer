use super::*;
use std::sync::Arc;

/*
    The CheckerTexture class implements the classic checkerboard texture
*/

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

// Constructors
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

/*
    Explanation: Why the following produces a checkered pattern

    The sines function below is a function that basically tesselates space into cubes where it's valued positively or negatively.
    The pattern produced on a surface comes from intersecting it with these cubes and taking the color assigned to the cube at the intersection
    If you can picture it in your head: think a checkerboard pattern, but with cubes (color changes whenever you move through the side of a cube).
    This function is basically partitioning space into such a checkerboard, and when you sample the color of a surface it takes on the color
    of the cube the surface intersects at a point
*/
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
