mod vec3;

use vec3::Vec3;

fn main(){
    let a = Vec3::new(1f32,2f32,3f32);
    let mut b = -a;
    b[0] = 7f32;
    println!("{} {} {}", b[0], b[1], b[2]);
}
