use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin : Vec3,
    pub lower_left_corner : Vec3,
    pub horizontal : Vec3,
    pub vertical : Vec3,
}

//constructors

impl Camera {
    pub fn new(llc : Vec3, h : Vec3, v : Vec3, o : Vec3) -> Camera {
        Camera {origin : o, lower_left_corner : llc, horizontal : h, vertical : v}
    }
    //default for our 200x100 small images
    pub fn default() -> Camera {
        Self::new(Vec3::new(-2.0, -1.0, -1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, 0.0))
    }
}

//methods

impl Camera {
    //returns a Ray in normalized direction (u,v) assuming the screen is [0,1] x [0,1]
    pub fn get_ray(&self, u : f32, v : f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical)
    }
}

