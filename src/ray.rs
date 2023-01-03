use crate::vec3::Vec3;


#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
   pub origin: Vec3,
   pub direction: Vec3,
}

//constructors
impl Ray {
    pub fn new(a : Vec3, b : Vec3) -> Ray{
        Ray {origin : a, direction : b}
    }
}

//getters deemed unnecessary as of now

impl Ray {
    //returns point when moving t times the direction from the origin
    pub fn trace(&self, t : f32) -> Vec3 {
       self.origin + t * self.direction 
    }
}
