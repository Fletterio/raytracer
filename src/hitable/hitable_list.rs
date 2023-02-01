use super::{
    material::{lambertian::Lambertian, Material},
    HitRecord, Hitable,
};
use crate::rtweekend::{Point3, Ray, Vec3};
use std::sync::Arc;

pub struct HitableList {
    pub list: Vec<Arc<dyn Hitable>>,
}

//constructors

impl HitableList {
    pub fn new(l: Vec<Arc<dyn Hitable>>) -> HitableList {
        HitableList { list: l }
    }
}

//methods
impl HitableList {
    //add objects to list
    pub fn add(&mut self, object: Arc<dyn Hitable>) {
        self.list.push(object);
    }

    //clear the list
    pub fn clear(&mut self) {
        self.list.clear();
    }
}

//Hitable Lists are Hitable

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut actual_hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for hitable in self.list.iter() {
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
