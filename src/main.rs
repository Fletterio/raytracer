mod vec3;

use vec3::Vec3;

fn main(){
    let a = Vec3::new(1f32,2f32,3f32);
    let b = Vec3::new(2f32, 2f32, 3f32);
    println!("{}", Vec3::dot(a,b));
    println!("{:?}", Vec3::cross(a,b));
}
