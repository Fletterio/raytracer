use crate::hitable::{material::Material, HitRecord};
use crate::rtweekend::{Color, Point3, Ray};
use crate::texture::{SolidColor, Texture};
use std::sync::Arc;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

//constructors
impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        DiffuseLight { emit: texture }
    }

    pub fn from_color(c: Color) -> Self {
        DiffuseLight {
            emit: Arc::new(SolidColor::from(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
