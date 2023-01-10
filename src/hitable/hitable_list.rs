use super::{Hitable, HitRecord, material::{Material, lambertian::Lambertian}};
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct HitableList <'a> {
    pub list : Vec<Box<dyn Hitable + 'a>>
}

//constructors

impl <'a> HitableList <'a> {
    pub fn new (l : Vec<Box<dyn Hitable + 'a>>) -> HitableList<'a> {
        HitableList {list : l}
    }
}

//Hitable Lists are Hitable

impl <'a> Hitable for HitableList<'a> {
    fn hit(&self, r : &Ray, t_min : f32, t_max : f32) -> Option<HitRecord>{
        let mut actual_hit_record : Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for hitable in self.list.iter(){
            if let Some(hit_record) = hitable.hit(r, t_min, closest_so_far) {
                //rec.t = hit_record.t;
                //rec.p = hit_record.p;
                //rec.normal = hit_record.normal;
                //rec.material = Rc::clone(&hit_record.material);
                closest_so_far = hit_record.t;
                actual_hit_record = Some(hit_record);
            }
        }
        return actual_hit_record;
    }
}
