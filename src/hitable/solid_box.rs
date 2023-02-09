use std::sync::Arc;

use crate::{aabb::AABB, rtweekend::Ray, vec3::Point3};

use super::{
    hitable_list::HitableList,
    material::Material,
    rectangles::{XYRect, XZRect, YZRect},
    HitRecord, Hitable,
};

pub struct SolidBox {
    box_min: Point3,
    box_max: Point3,
    sides: HitableList,
}

//constructors
impl SolidBox {
    pub fn new(p0: Point3, p1: Point3, material: Arc<dyn Material>) -> Self {
        let mut sides_ = HitableList::new(vec![]);
        let [x0, y0, z0] = p0.as_array();
        let [x1, y1, z1] = p1.as_array();

        sides_.add(Arc::new(XYRect::new(
            x0,
            x1,
            y0,
            y1,
            z1,
            Arc::clone(&material),
        )));
        sides_.add(Arc::new(XYRect::new(
            x0,
            x1,
            y0,
            y1,
            z0,
            Arc::clone(&material),
        )));

        sides_.add(Arc::new(XZRect::new(
            x0,
            x1,
            z0,
            z1,
            y1,
            Arc::clone(&material),
        )));
        sides_.add(Arc::new(XZRect::new(
            x0,
            x1,
            z0,
            z1,
            y0,
            Arc::clone(&material),
        )));

        sides_.add(Arc::new(YZRect::new(
            y0,
            y1,
            z0,
            z1,
            x1,
            Arc::clone(&material),
        )));
        sides_.add(Arc::new(YZRect::new(
            y0,
            y1,
            z0,
            z1,
            x0,
            Arc::clone(&material),
        )));

        SolidBox {
            box_min: p0,
            box_max: p1,
            sides: sides_,
        }
    }
}

impl Hitable for SolidBox {
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}
