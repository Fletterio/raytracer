// This module contains usual unary/binary operations on Vectors (mostly addition/subtraction and indexing,
// product and division as implemented here are useful only for colour operations) and some other utility functions

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use super::*;

//-------------------------Operators-------------------------------------

// negation
impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3 { e: -self.e }
    }
}

// Probably not used much, just allows to access a vector like a tuple or array
impl Index<usize> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

// Same as before but produces a mutable value
impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

// Vector addition
impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            e: self.e + other.e,
        }
    }
}

//Same as before, with assignment
impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.e += other.e;
    }
}

// Vector subtraction
impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            e: self.e - other.e,
        }
    }
}

// Same as before, with assignment
impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.e -= other.e;
    }
}

// Coordinate-wise multiplication
impl Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            e: self.e * other.e,
        }
    }
}

// Same as before, with assignment
impl MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.e *= other.e;
    }
}

// Coordinate-wise division. Probably panics on division by zero
impl Div for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        let f = other.e + f32x4::from_array([0.0, 0.0, 0.0, 1.0]);
        Self { e: self.e / f }
    }
}

// Same as before, with assignment
impl DivAssign for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        let f = other.e + f32x4::from_array([0.0, 0.0, 0.0, 1.0]);
        self.e /= f;
    }
}
