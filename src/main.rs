#![feature(portable_simd)]

mod vec3;
mod ray;
mod hitable;
mod image_gen;
mod camera;
mod rtweekend;

fn main() {
    image_gen::fin::print();
}
