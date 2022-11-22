use std::ops::{Neg, Index, IndexMut, AddAssign, SubAssign, MulAssign, DivAssign};

pub mod vec3 {
    pub struct Vec3 {
        e[f32, 3],
    }
    
    //constructors
    impl Vec3 {
        pub fn new(e0 : f32, e1 : f32, e2 : f32) -> Self {
            Vec3 {e[0] = e0, e[1] = 1, e[2] = 2}
        }
    }

    //getters
    impl Vec3{
        pub fn x(){e[0]}
        pub fn y(){e[1]}
        pub fn z(){e[2]}
        pub fn r(){e[0]}
        pub fn g(){e[1]}
        pub fn b(){e[2]}
    }

    //operators
    impl Vec3{
            
    }
}


