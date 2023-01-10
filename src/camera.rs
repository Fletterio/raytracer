use crate::rtweekend::{Vec3, Point3, Ray};

pub struct Camera {
    pub origin : Point3,
    pub lower_left_corner : Vec3,
    pub horizontal : Vec3,
    pub vertical : Vec3,
}

//constructors

impl Camera {
    pub fn new(llc : Vec3, h : Vec3, v : Vec3, o : Vec3) -> Camera {
        Camera {origin : o, lower_left_corner : llc, horizontal : h, vertical : v}
    }
    //default for our project
    pub fn default() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - 0.5 * horizontal - 0.5 * vertical - Vec3::new(0.0, 0.0, focal_length);
        return Self::new(lower_left_corner, horizontal, vertical, origin);
    }
}

//methods

impl Camera {
    //returns a Ray in normalized direction (u,v) assuming the screen is [0,1] x [0,1]
    pub fn get_ray(&self, u : f32, v : f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical)
    }
}

