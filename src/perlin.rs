use crate::rtweekend::{random_double, random_int, Point3};
use std::mem::swap;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    pub ranfloat: [f32; POINT_COUNT],
    pub perm_x: [i32; POINT_COUNT],
    pub perm_y: [i32; POINT_COUNT],
    pub perm_z: [i32; POINT_COUNT],
}

//constructors
impl Perlin {
    pub fn new() -> Self {
        let mut rf: [f32; POINT_COUNT];
        for f in rf.iter_mut() {
            *f = random_double();
        }
        let px = perlin_generate_perm();
        let py = perlin_generate_perm();
        let pz = perlin_generate_perm();

        Perlin {
            ranfloat: rf,
            perm_x: px,
            perm_y: py,
            perm_z: pz,
        }
    }
}

//generate moise
impl Perlin {
    pub fn noise(&self, p: &Point3) -> f32 {
        let [x, y, z] = p.as_array();
        let i = (4.0 * x).round() as i32 & 255;
        let j = (4.0 * y).round() as i32 & 255;
        let k = (4.0 * z).round() as i32 & 255;
        self.ranfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }
}

fn perlin_generate_perm() -> [i32; POINT_COUNT] {
    let mut p: [i32; POINT_COUNT];

    for i in 0..POINT_COUNT {
        p[i] = i as i32;
    }

    permute(&mut p);

    p
}

fn permute(p: &mut [i32]) {
    for i in (1..p.len()).rev() {
        let target = random_int(0, i as i32) as usize;
        swap(&mut p[i], &mut p[target])
    }
}
