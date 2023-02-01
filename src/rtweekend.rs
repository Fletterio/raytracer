//general utility file

use rand::distributions::{Distribution, Uniform};
use std::simd::f32x4;

//Constants
pub const PI: f32 = std::f32::consts::PI;
pub const INFINITY: f32 = f32::INFINITY;
pub const DELTA: f32 = 0.001; //points closer than this are essentially the same
                              //this avoids multiple hits on the same position when
                              //a ray spawned at an object hits the same object because
                              //of numerical inaccuracy at small values.

pub const UP: Vec3 = Vec3 {
    e: f32x4::from_array([0.0, 1.0, 0.0, 0.0]),
}; //up in the y direction

//Utility

#[inline]
pub fn deg_to_rad(deg: f32) -> f32 {
    return deg * PI / 180.0;
}

#[inline]
pub fn random_double() -> f32 {
    return random_double_between(0.0, 1.0);
}

#[inline]
pub fn random_double_between(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(min..max);
    return uniform.sample(&mut rng);
}

#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    return x;
}

//Common headers

pub use crate::ray::Ray;
pub use crate::vec3::{Color, Point3, Vec3};
