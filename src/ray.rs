use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f32,
}

//constructors
impl Ray {
    #[inline]
    pub fn new(a: Vec3, b: Vec3, tm: f32) -> Ray {
        Ray {
            origin: a,
            direction: b,
            time: tm,
        }
    }
}

//getters deemed unnecessary as of now

impl Ray {
    //returns point when moving t times the direction from the origin
    #[inline]
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
