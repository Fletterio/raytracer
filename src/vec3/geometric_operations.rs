use super::*;
use std::ops::{Div, DivAssign, Mul, MulAssign};
use std::simd::simd_swizzle;
use std::simd::SimdFloat;

impl Vec3 {
    #[inline]
    pub fn dot(v1: &Self, v2: &Self) -> f32 {
        (v1.e * v2.e).reduce_sum()
    }

    // https://geometrian.com/programming/tutorials/cross-product/index.php
    #[inline]
    pub fn cross(v0: &Self, v1: &Self) -> Self {
        let t0: f32x4 = simd_swizzle!(v0.e, [1, 2, 0, 3]);
        let t1: f32x4 = simd_swizzle!(v1.e, [2, 0, 1, 3]);
        let t2: f32x4 = t0 * v1.e;
        let t3: f32x4 = t0 * t1;
        let t4: f32x4 = simd_swizzle!(t2, [1, 2, 0, 3]);
        Self { e: t3 - t4 }
    }

    #[inline]
    pub fn normalize(v1: Self) -> Self {
        (1f32 / v1.len()) * v1
    }

    #[inline]
    pub fn len(&self) -> f32 {
        return f32::sqrt(Self::dot(&self, &self));
    }

    #[inline]
    pub fn sq_len(&self) -> f32 {
        return Self::dot(&self, &self);
    }
}

//ray interaction functions
impl Vec3 {
    #[inline]
    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2f32 * Vec3::dot(v, n) * *n
    }

    #[inline]
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = Vec3::dot(&-*uv, n).min(1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -f32::sqrt((1.0 - r_out_perp.sq_len()).abs()) * *n;
        return r_out_perp + r_out_parallel;
    }
}
//scaling
//we adopt the convention that multiples of vectors are written in the form a*v, a being a scalar
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        let d = Vec3 {
            e: f32x4::splat(self),
        };
        v * d
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, f: f32) -> Vec3 {
        let d = f32x4::splat(f);
        Vec3 { e: self.e / d }
    }
}

//same as before but on assign

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, f: f32) {
        let d = Vec3 { e: f32x4::splat(f) };
        *self *= d;
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, f: f32) {
        let d = Vec3 { e: f32x4::splat(f) };
        *self /= d;
    }
}
