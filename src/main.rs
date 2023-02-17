#![feature(portable_simd)]

use std::env;

mod aabb;
mod bvh_node;
mod camera;
mod hitable;
mod image_gen;
mod perlin;
mod ray;
mod renderer;
mod rtweekend;
mod texture;
mod vec3;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    image_gen::switch::print();
}
