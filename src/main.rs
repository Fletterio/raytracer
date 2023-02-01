#![feature(portable_simd)]
mod camera;
mod hitable;
mod image_gen;
mod ray;
mod renderer;
mod rtweekend;
mod vec3;

fn main() {
    image_gen::fin::print();
}
