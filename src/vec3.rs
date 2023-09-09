use std::simd::f32x4;

pub mod color;
pub mod geometric_operations;
pub mod operations;

use crate::rtweekend::random_double_between;
use std::fmt;

/*
    The vec3 class represents a 3D vector using SIMD operations for optimized performance with common operations
    The fourth coordinate remains unused since all math used here is strictly 3D (no use of quaternions)
*/

//------------------- NOTE for reading this class' methods --------------------------
// Whenever a vector argument is called 'n' it means the parameter MUST be normalized

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vec3 {
    pub e: f32x4, //__m128 if you're a cpp person
}

//--------------------------- Constructors ---------------------------
//             They take the vector's coordinates as input
impl Vec3 {
    #[inline]
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 {
            e: f32x4::from_array([e0, e1, e2, 0.0]),
        }
    }

    pub fn from_array(a: [f32; 3]) -> Self {
        Vec3 {
            e: f32x4::from_array([a[0], a[1], a[2], 0.0]),
        }
    }
}

//----------------------------- Getters ---------------------------

// WARNING: packed vectors like these are not meant to be read often. Keep the use of getters to a minimum!
// Consider instead using the SIMD getter specified right below
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

// SIMD-efficient getter
impl Vec3 {
    // Reading into a SIMD is expensive. When we need to read all three value, we might as well just get them all in one read to the stack
    pub fn as_array(&self) -> [f32; 3] {
        let [x, y, z, w] = self.e.as_array();
        [*x, *y, *z]
    }
}

//--------------------------- Random Vector Generation -----------------------------
impl Vec3 {
    // Generates a vector with three uniform components in the [min,max) range
    #[inline]
    pub fn random_between(min: f32, max: f32) -> Vec3 {
        Self::new(
            random_double_between(min, max),
            random_double_between(min, max),
            random_double_between(min, max),
        )
    }

    // Same as before but specifically in [0,1)
    #[inline]
    pub fn random() -> Vec3 {
        Vec3::random_between(0.0, 1.0)
    }

    // Returns a random vector in the (open) unit sphere
    #[inline]
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::random_between(-1.0, 1.0);
            if p.sq_len() < 1.0 {
                return p;
            }
        }
    }

    // Returns a random unit direction in S^2
    #[inline]
    pub fn random_unit_vector() -> Vec3 {
        return Self::normalize(Self::random_in_unit_sphere());
    }

    // Returns a random unit direction in the same hemisphere as the argument n
    #[inline]
    pub fn random_in_hemisphere(n: &Vec3) -> Vec3 {
        let mut in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(&in_unit_sphere, n) < 0.0 {
            in_unit_sphere = -in_unit_sphere; //If it's in the other hemisphere we simply flip it
        }
        return in_unit_sphere;
    }

    // Returns a Vec3 representing a unit direction in S^1
    #[inline]
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_double_between(-1.0, 1.0),
                random_double_between(-1.0, 1.0),
                0.0,
            );
            if p.sq_len() < 1.0 {
                return p;
            };
        }
    }
}

//--------------------------- Misc Functions ---------------------------

impl Vec3 {
    // Checks if a vector is too small
    #[inline]
    pub fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        return self.e[0].abs() < S && self.e[1].abs() < S && self.e[2].abs() < S;
    }
}

//-------------------------- Semantic Type Aliases ----------------------
// Basically used only to differentiate different uses of the Vec3 class

pub type Point3 = Vec3;
pub type Color = Vec3;

// Implementing a display trait to be able to print to stderr

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
