use std::ops::{Neg, Index, IndexMut, AddAssign, SubAssign, MulAssign, DivAssign};

pub mod vec3 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct Vec3 {
        e : [f32; 3],
    }
    
    //constructors
    impl Vec3 {
        pub fn new(e0 : f32, e1 : f32, e2 : f32) -> Self {
            Vec3 {e : [e0, e1, e2]}
        }
    }

    //getters
    impl Vec3{
        pub fn x(self){self.e[0]}
        pub fn y(self){self.e[1]}
        pub fn z(self){self.e[2]}
        pub fn r(self){self.e[0]}
        pub fn g(self){self.e[1]}
        pub fn b(self){self.e[2]}
    }

    //operators
    impl Neg for Vec3{
        type Output = Self;

        fn neg(self) -> Self::Output{
            Vec3{e : [-self.e[0], -self.e[1], -self.e[2]]}
        }
    }
}


