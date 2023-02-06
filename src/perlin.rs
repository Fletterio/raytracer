use crate::rtweekend::{random_double, random_int, Point3};
use std::mem::swap;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    pub ranfloat: [f32; POINT_COUNT],
    pub perm_x: [usize; POINT_COUNT],
    pub perm_y: [usize; POINT_COUNT],
    pub perm_z: [usize; POINT_COUNT],
}

//constructors
impl Perlin {
    pub fn new() -> Self {
        let mut rf: [f32; POINT_COUNT] = [0.0; POINT_COUNT];
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

        let u = x - x.floor();
        let v = y - y.floor();
        let w = z - z.floor();

        let i = x.floor() as i32;
        let j = y.floor() as i32;
        let k = z.floor() as i32;

        let mut c = [[[0f32; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        trilinear_interpolation(c, u, v, w)
    }
}

fn perlin_generate_perm() -> [usize; POINT_COUNT] {
    let mut p: [usize; POINT_COUNT] = [0; POINT_COUNT];

    for i in 0..POINT_COUNT {
        p[i] = i;
    }

    permute(&mut p);

    p
}

fn permute(p: &mut [usize]) {
    for i in (1..p.len()).rev() {
        let target = random_int(0, i as i32) as usize;
        p.swap(i, target);
    }
}

fn trilinear_interpolation(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f32 * u + (1.0 - i as f32) * (1.0 - u))
                    * (j as f32 * v + (1.0 - j as f32) * (1.0 - v))
                    * (k as f32 * w + (1.0 - k as f32) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }
    accum
}
