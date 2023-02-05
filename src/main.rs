#![feature(portable_simd)]
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
    image_gen::fin::print();
}
