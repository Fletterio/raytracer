use super::Vec3;
use std::ops::Mul;

impl Vec3{
    pub fn dot(v1 : Self, v2 : Self) -> f32 {
        v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
    }

    pub fn cross(v1 : Self, v2 : Self) -> Self {
        Self{ e : [v1[1] * v2[2] - v1[2] * v2[1], 
                   v1[2] * v2[0] - v1[0] * v2[2],
                   v1[0] * v2[1] - v1[1] * v2[0]]}
    }

    pub fn normalize(v1 : Self) -> Self{
        (1f32 / v1.len()) * v1        
    }

    pub fn len(self) -> f32{
        return f32::sqrt(Self::dot(self, self));
    }
}


//scaling

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec : Vec3) -> Vec3{
        Vec3 {e : [vec[0] * self, vec[1] * self, vec[2] * self],}
    }
}
