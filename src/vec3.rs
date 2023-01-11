pub mod operations;
pub mod geometric_operations;
pub mod color;

use std::fmt;
use crate::rtweekend::{random_double_between};

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
impl Color {
    pub fn gamma_correct(&mut self, alpha : f32) {
        self.e = [self.e[0].powf(alpha), self.e[1].powf(alpha), self.e[2].powf(alpha)];
    }
}

//random vectors
impl Vec3 {
    pub fn random_between(min : f32, max : f32) -> Vec3 {
        Self::new(random_double_between(min, max), random_double_between(min, max), random_double_between(min, max))
    }

    pub fn random() -> Vec3 {
        Vec3::random_between(0.0, 1.0)
    }

    //returns a random vector in the (open) unit sphere
    pub fn random_in_unit_sphere() -> Vec3 {
        while true {
            let p = Self::random_between(-1.0, 1.0);
            if p.sq_len() < 1.0 {return p;}
        }
        return Self::new(0.0, 0.0, 0.0);
    }
    pub fn random_unit_vector() -> Vec3 {
        return Self::normalize(Self::random_in_unit_sphere());
    }

    pub fn random_in_hemisphere(n : &Vec3) -> Vec3 {
        let mut in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(&in_unit_sphere, n) < 0.0 {
            in_unit_sphere = - in_unit_sphere;
        }
        return in_unit_sphere;
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


