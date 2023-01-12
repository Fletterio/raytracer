use super::Vec3;
use std::ops::{Mul, Div, MulAssign, DivAssign};

impl Vec3{
    #[inline]
    pub fn dot(v1 : &Self, v2 : &Self) -> f32 {
        v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
    }

    #[inline]
    pub fn cross(v1 : &Self, v2 : &Self) -> Self {
        Self{ e : [v1[1] * v2[2] - v1[2] * v2[1], 
                   v1[2] * v2[0] - v1[0] * v2[2],
                   v1[0] * v2[1] - v1[1] * v2[0]]}
    }

    #[inline]
    pub fn normalize(v1 : Self) -> Self{
        (1f32 / v1.len()) * v1        
    }

    #[inline]
    pub fn len(&self) -> f32{
        return f32::sqrt(Self::dot(&self, &self));
    }

    #[inline]
    pub fn sq_len(&self) -> f32{
        return Self::dot(&self,&self);
    }
}

//ray interaction functions
impl Vec3 {
    #[inline]
    pub  fn reflect(v : &Vec3, n : &Vec3) -> Vec3 {
        *v - 2f32 * Vec3::dot(v,n) * *n
    }

    #[inline]
    pub fn refract(uv : &Vec3, n : &Vec3, etai_over_etat : f32) -> Vec3{
        let cos_theta = Vec3::dot(&-*uv, n).min(1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = - f32::sqrt((1.0 - r_out_perp.sq_len()).abs()) * *n;
        return r_out_perp + r_out_parallel;
    }

}
//scaling
//we adopt the convention that multiples of vectors are written in the form a*v, a being a scalar
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v : Vec3) -> Vec3{
        Vec3 {e : [v[0] * self, v[1] * self, v[2] * self],}
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, f : f32) -> Vec3 {
        Vec3 {e : [self[0] / f, self[1] / f, self[2] / f],}
    }
}

//same as before but on assign

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, f : f32){
        self.e[0] *= f;
        self.e[1] *= f;
        self.e[2] *= f;
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, f : f32){
        self.e[0] /= f;
        self.e[1] /= f;
        self.e[2] /= f;
    }
}
