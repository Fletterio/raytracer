mod vec3;

use vec3::vec3::Vec3;

fn main(){
    let a = Vec3::new(1,2,3);
    let b = -a;
    println!("{}", b);
}
