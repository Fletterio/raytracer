use std::simd::f32x4;

pub mod color;
pub mod geometric_operations;
pub mod operations;

use crate::rtweekend::random_double_between;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vec3 {
    pub e: f32x4,
}

//constructors
impl Vec3 {
    #[inline]
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 {
            e: f32x4::from_array([e0, e1, e2, 0.0]),
        }
    }
}

//getters
impl Vec3 {
    #[inline]
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.e[2]
    }
    #[inline]
    pub fn r(&self) -> f32 {
        self.e[0]
    }
    #[inline]
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    #[inline]
    pub fn b(&self) -> f32 {
        self.e[2]
    }
}

//colour functions
impl Color {
    #[inline]
    pub fn gamma_correct(&mut self, alpha: f32) {
        let self_as_array = self.as_array();
        self.e = f32x4::from_array(
            self_as_array
                .into_iter()
                .map(|x| x.powf(alpha))
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        );
    }
}

//random vectors
impl Vec3 {
    #[inline]
    pub fn random_between(min: f32, max: f32) -> Vec3 {
        Self::new(
            random_double_between(min, max),
            random_double_between(min, max),
            random_double_between(min, max),
        )
    }

    #[inline]
    pub fn random() -> Vec3 {
        Vec3::random_between(0.0, 1.0)
    }

    //returns a random vector in the (open) unit sphere
    #[inline]
    pub fn random_in_unit_sphere() -> Vec3 {
        while true {
            let p = Self::random_between(-1.0, 1.0);
            if p.sq_len() < 1.0 {
                return p;
            }
        }
        return Self::new(0.0, 0.0, 0.0);
    }
    #[inline]
    pub fn random_unit_vector() -> Vec3 {
        return Self::normalize(Self::random_in_unit_sphere());
    }

    #[inline]
    pub fn random_in_hemisphere(n: &Vec3) -> Vec3 {
        let mut in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(&in_unit_sphere, n) < 0.0 {
            in_unit_sphere = -in_unit_sphere;
        }
        return in_unit_sphere;
    }

    #[inline]
    pub fn random_in_unit_disk() -> Vec3 {
        while true {
            let p = Vec3::new(
                random_double_between(-1.0, 1.0),
                random_double_between(-1.0, 1.0),
                0.0,
            );
            if p.sq_len() < 1.0 {
                return p;
            };
        }
        return Self::new(0.0, 0.0, 0.0);
    }
}

//check for very small vectors
impl Vec3 {
    #[inline]
    pub fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        return self.e[0].abs() < S && self.e[1].abs() < S && self.e[2].abs() < S;
    }
}

//type aliases

pub type Point3 = Vec3;
pub type Color = Vec3;

//printing

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

//-----------------------------------------SIMD ONLY--------------------

impl Vec3 {
    //reading into a SIMD is expensive. When we need to read all three value, we might as well just get them all in one read to the stack
    pub fn as_array(&self) -> [f32; 3] {
        let [x, y, z, w] = self.e.as_array();
        [*x, *y, *z]
    }
}
