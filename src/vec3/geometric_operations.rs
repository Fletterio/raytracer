use super::Vec3;


impl Vec3{
    pub fn dot(v1 : Self, v2 : Self) -> f32 {
        v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
    }

    pub fn cross(v1 : Self, v2 : Self) -> Self {
        Self{ e : [v1[1] * v2[2] - v1[2] * v2[1], 
                   v1[2] * v2[0] - v1[0] * v2[2],
                   v1[0] * v2[1] - v1[1] * v2[0]]}
    }
}
