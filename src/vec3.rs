pub mod operations;
pub mod geometric_operations;
pub mod color;

use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vec3 {
   pub e: [f32; 3],
}

//constructors
impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }
}

//getters
impl Vec3 {
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }
    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }
}

//colour functions
impl Vec3 {
    pub fn gamma_correct(&mut self, alpha : f32) {
        self.e = [self.e[0].powf(alpha), self.e[1].powf(alpha), self.e[2].powf(alpha)];
    }
}

//type aliases

pub type Point3 = Vec3;
pub type Color = Vec3;

//printing

impl fmt::Display for Vec3 {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}


