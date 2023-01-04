use super::{Hitable, HitRecord};
use crate::ray::Ray;

pub struct HitableList <'a> {
    list : &'a mut Vec<&'a dyn Hitable>,
}

//constructors

impl <'a> HitableList <'a> {
    pub fn new (l : &'a mut Vec<&'a dyn Hitable>) -> HitableList {
        HitableList {list : l}
    }
}

//Hitable Lists are Hitable

impl <'a> Hitable for HitableList<'a> {
    fn hit(&self, r : &Ray, t_min : f32, t_max : f32) -> HitRecord {
        let hit_anything = false;
        let mut hit_record = HitRecord {hit : false, t : 0f32, p : r.origin, normal : r.origin};
        let mut closest_so_far = t_max;
        for hitable in self.list.iter() {
            hit_record = hitable.hit(r, t_min, closest_so_far);
            if hit_record.hit {
                closest_so_far = hit_record.t;
            }
        }
        return hit_record;
    }
}
