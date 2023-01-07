use super::Vec3;
use std::ops::{Mul, Div, MulAssign, DivAssign};

impl Vec3{
    pub fn dot(v1 : &Self, v2 : &Self) -> f32 {
        v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
    }

    pub fn cross(v1 : &Self, v2 : &Self) -> Self {
        Self{ e : [v1[1] * v2[2] - v1[2] * v2[1], 
                   v1[2] * v2[0] - v1[0] * v2[2],
                   v1[0] * v2[1] - v1[1] * v2[0]]}
    }

    pub fn normalize(v1 : Self) -> Self{
        (1f32 / v1.len()) * v1        
    }

    pub fn len(&self) -> f32{
        return f32::sqrt(Self::dot(&self, &self));
    }

    pub fn sq_len(&self) -> f32{
        return Self::dot(&self,&self);
    }
}


//scaling
//we adopt the convention that multiples of vectors are written in the form a*v, a being a scalar
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v : Vec3) -> Vec3{
        Vec3 {e : [v[0] * self, v[1] * self, v[2] * self],}
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, f : f32) -> Vec3 {
        Vec3 {e : [self[0] / f, self[1] / f, self[2] / f],}
    }
}

//same as before but on assign

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, f : f32){
        self.e[0] *= f;
        self.e[1] *= f;
        self.e[2] *= f;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, f : f32){
        self.e[0] /= f;
        self.e[1] /= f;
        self.e[2] /= f;
    }
}
