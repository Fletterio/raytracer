use crate::rtweekend::{Vec3, Point3, Ray, deg_to_rad};

pub struct Camera {
    pub origin : Point3,
    pub lower_left_corner : Vec3,
    pub horizontal : Vec3,
    pub vertical : Vec3,
    pub u : Vec3,
    pub v : Vec3,
    pub w : Vec3,
    pub lens_radius : f32,
}

//constructors

impl Camera {
    pub fn new(lookfrom : Point3, 
               lookat : Point3, 
               vup : Vec3, //vup is the orientation up vector
               vfov : f32, //vertical field of view, in degrees
               aspect_ratio : f32,
               aperture : f32,  //aperture radius of the simulated camera
               focus_dist : f32 //distance of the focus plane from the camera
               ) -> Camera 
    {    
        let theta = deg_to_rad(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::normalize(lookfrom - lookat); //ensure they're not too close
        let u = Vec3::normalize(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w,&u);

        let o = lookfrom;
        let h = focus_dist * viewport_width * u;
        let v = focus_dist * viewport_height * v;
        let llc = o - 0.5 * h - 0.5 * v - focus_dist * w;
        let lr = aperture / 2.0;

        Camera {origin : o, lower_left_corner : llc, horizontal : h, vertical : v, u : u, v : v, w : w, lens_radius : lr}
    }

}

//methods

impl Camera {
    //returns a Ray in normalized direction (s,t) assuming the screen is [0,1] x [0,1]
    pub fn get_ray(&self, s : f32, t : f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = rd.x() * self.u + rd.y() * self.v;

        let ray_origin = self.origin + offset;

        return Ray::new(
            ray_origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - ray_origin
        );
    }
}

