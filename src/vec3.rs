use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, Neg, SubAssign};
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    e: [f32; 3],
}

//constructors
impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }
}

//getters
impl Vec3 {
    pub fn x(self) -> f32 {
        self.e[0]
    }
    pub fn y(self) -> f32 {
        self.e[1]
    }
    pub fn z(self) -> f32 {
        self.e[2]
    }
    pub fn r(self) -> f32 {
        self.e[0]
    }
    pub fn g(self) -> f32 {
        self.e[1]
    }
    pub fn b(self) -> f32 {
        self.e[2]
    }
}

//operators
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Index<usize> for Vec3{
    type Output = f32;

    fn index(&self, index : usize) -> &Self::Output{
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index : usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}
