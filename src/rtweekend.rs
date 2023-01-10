//general utility file

use rand::distributions::{Distribution, Uniform};

//Constants
pub const PI : f32 = std::f32::consts::PI;
pub const INFINITY : f32 = f32::INFINITY;

//Utility

pub fn deg_to_rad(deg : f32) -> f32{
    return deg * PI / 180.0;
}

pub fn random_double() -> f32 {
    return random_double_between(0.0, 1.0);
}

pub fn random_double_between(min : f32, max : f32) -> f32 {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(min..max);
    return uniform.sample(&mut rng);
}

pub fn clamp(x : f32, min : f32, max : f32) -> f32 {
    if x < min {return min};
    if x > max {return max};
    return x;
}

//Common headers

pub use crate::vec3::{Vec3, Color, Point3};
pub use crate::ray::Ray;
