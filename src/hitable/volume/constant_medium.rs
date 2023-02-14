use std::sync::Arc;

use crate::{
    aabb::AABB,
    hitable::{
        material::{isotropic::Isotropic, Material},
        HitRecord, Hitable,
    },
    rtweekend::{random_double, Ray},
    texture::Texture,
    vec3::{Color, Vec3},
};

pub struct ConstantMedium {
    boundary: Arc<dyn Hitable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f32,
}

//constructors
impl ConstantMedium {
    pub fn new(b: Arc<dyn Hitable>, density: f32, text: Arc<dyn Texture>) -> Self {
        ConstantMedium {
            boundary: b,
            neg_inv_density: (-1.0 / density),
            phase_function: Arc::new(Isotropic::new(text)),
        }
    }

    pub fn from_color(b: Arc<dyn Hitable>, density: f32, c: Color) -> Self {
        ConstantMedium {
            boundary: b,
            neg_inv_density: (-1.0 / density),
            phase_function: Arc::new(Isotropic::from_color(c)),
        }
    }
}

impl Hitable for ConstantMedium {
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        //print samples to stderr for debugging purposes
        const enableDebug: bool = false;
        let debugging: bool = enableDebug && random_double() < 0.00001;

        let rec1 = self.boundary.hit(r, -f32::INFINITY, f32::INFINITY);
        if rec1.is_none() {
            return None;
        }
        let mut rec1 = rec1.unwrap();

        let rec2 = self.boundary.hit(r, rec1.t + 0.0001, f32::INFINITY);
        if rec2.is_none() {
            return None;
        }
        let mut rec2 = rec2.unwrap();

        if debugging {
            eprintln!("\nt_min = {}, t_max = {}", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return None;
        } //could happen due to switch

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction.len();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let hit_t = rec1.t + hit_distance / ray_length;
        let hit_p = r.at(hit_t);

        if debugging {
            eprintln!(
                "hit distance = {}\nrec.t = {}\nrec.p = {}",
                hit_distance, hit_t, hit_p
            );
        }

        let hit_normal = Vec3::new(1.0, 0.0, 0.0); //arbitrary
        let hit_material = Arc::clone(&self.phase_function);

        Some(HitRecord {
            p: hit_p,
            t: hit_t,
            normal: hit_normal,
            front_face: true, //won't change since normal is irrelevant
            material: hit_material,
            u: 0.0,
            v: 0.0,
        })
    }
}
