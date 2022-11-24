mod vec3;
mod simple_image;
use vec3::Vec3;

fn main(){
    println!("{:?}", Vec3::new(1f32, 2f32, 3f32));
    simple_image::print();
}
