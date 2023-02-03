use crate::aabb::AABB;
use crate::hitable::{HitRecord, Hitable};
use crate::rtweekend::{random_int, Ray};
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVH_Node {
    container: AABB,
    left: Arc<dyn Hitable>,
    right: Arc<dyn Hitable>,
}

//relaying constructors to the end since they're a bit more complicated

impl Hitable for BVH_Node {
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(self.container)
    }

    //hit is simple: If there's a hit, call hit on children nodes
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.container.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        match hit_left {
            None => self.right.hit(r, t_min, t_max),
            Some(hitLeftRecord) => {
                let hit_right = self.right.hit(r, t_min, hitLeftRecord.t);
                match hit_right {
                    None => Some(hitLeftRecord),
                    Some(rightHitRecord) => Some(rightHitRecord),
                }
            }
        }
    }
}

//constructors
impl BVH_Node {
    pub fn new(src_objects: &[impl Hitable], time0: f32, time1: f32) -> Self {
        let mut objects = src_objects.clone();
        let axis = random_int(0, 2) as usize;
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };
        let object_span = src_objects.len();
        let out_left: Arc<dyn Hitable>;
        let out_right: Arc<dyn Hitable>;
        if 1 == object_span {
            out_left = Arc::new(objects[0]);
            out_right = Arc::new(objects[0]);
        } else if 2 == object_span {
            if Ordering::Less == comparator(&objects[0], &objects[1]) {
                out_left = Arc::new(objects[0]);
                out_right = Arc::new(objects[1]);
            } else {
                out_left = Arc::new(objects[1]);
                out_right = Arc::new(objects[0]);
            }
        } else {
            objects.sort_by(comparator);

            let mid = object_span / 2;
            out_left = Arc::new(BVH_Node::new(&objects[0..mid], time0, time1));
            out_right = Arc::new(BVH_Node::new(&objects[mid..], time0, time1));
        }

        let box_left = out_left.bounding_box(time0, time1);
        let box_right = out_right.bounding_box(time0, time1);

        let out_box: AABB;
        match (box_left, box_right) {
            (None, _) | (_, None) => panic!("No bounding box in BVH_Node constructor \n"),
            (Some(b_l), Some(b_r)) => {
                out_box = AABB::surrounding_box(&b_l, &b_r);
            }
        };

        BVH_Node {
            container: out_box,
            left: out_left,
            right: out_right,
        }
    }
}

fn box_compare(a: &impl Hitable, b: &impl Hitable, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if let (None, _) | (_, None) = (box_a, box_b) {
        panic!("No bounding box in bvh_node constructor\n")
    }

    let a_min = box_a.unwrap().min.e[axis];
    let b_min = box_b.unwrap().min.e[axis];

    if a_min < b_min {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn box_x_compare(a: &impl Hitable, b: &impl Hitable) -> Ordering {
    box_compare(a, b, 0)
}
fn box_y_compare(a: &impl Hitable, b: &impl Hitable) -> Ordering {
    box_compare(a, b, 1)
}
fn box_z_compare(a: &impl Hitable, b: &impl Hitable) -> Ordering {
    box_compare(a, b, 2)
}
