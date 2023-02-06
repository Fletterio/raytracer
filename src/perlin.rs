use crate::rtweekend::{random_double, random_int, Point3, Vec3};
use std::mem::swap;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    pub ranvec: [Vec3; POINT_COUNT],
    pub perm_x: [usize; POINT_COUNT],
    pub perm_y: [usize; POINT_COUNT],
    pub perm_z: [usize; POINT_COUNT],
}

//constructors
impl Perlin {
    pub fn new() -> Self {
        let mut rv: [Vec3; POINT_COUNT] = [Vec3::new(0.0, 0.0, 0.0); POINT_COUNT];
        for v in rv.iter_mut() {
            *v = Vec3::normalize(Vec3::random_between(-1.0, 1.0));
        }
        let px = perlin_generate_perm();
        let py = perlin_generate_perm();
        let pz = perlin_generate_perm();

        Perlin {
            ranvec: rv,
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

        /*let u = u * u * (3.0 - 2.0 * u); //Hermitian Smoothing
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w); */

        let i = x.floor() as i32;
        let j = y.floor() as i32;
        let k = z.floor() as i32;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        perlin_interp(c, u, v, w)
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

//old, deprecated
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

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u); //Hermitian Smoothing
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * v);
    let mut accum = 0.0;

    for i in 0..2 {
        let fi = i as f32;
        for j in 0..2 {
            let fj = j as f32;
            for k in 0..2 {
                let fk = k as f32;
                let weight_v = Vec3::new(u - fi, v - fj, w - fk);

                accum += (fi * uu + (1.0 - fi) * (1.0 - uu))
                    * (fj * vv + (1.0 - fj) * (1.0 - vv))
                    * (fk * ww + (1.0 - fk) * (1.0 - ww))
                    * Vec3::dot(&c[i][j][k], &weight_v)
            }
        }
    }
    accum
}
