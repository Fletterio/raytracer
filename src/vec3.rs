pub mod operations;
pub mod geometric_operations;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
   pub e: [f32; 3],
}

//constructors
impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }
}

//getters
impl Vec3 {
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }
    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }
}
