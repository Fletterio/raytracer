use super::{Hitable, HitRecord};
use crate::ray::Ray;

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
    fn hit(&self, r : &Ray, t_min : f32, t_max : f32) -> HitRecord {
        let mut hit_record = HitRecord {hit : false, t : 0f32, p : r.origin, normal : r.origin};
        let mut closest_so_far = t_max;
        for hitable in self.list.iter() {
            let temp_hit_record = hitable.hit(r, t_min, closest_so_far);
            if temp_hit_record.hit {
                closest_so_far = temp_hit_record.t;
                hit_record = temp_hit_record;
            }
        }
        return hit_record;
    }
}
