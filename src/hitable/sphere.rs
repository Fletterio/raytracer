use super::{material::Material, HitRecord, Hitable, AABB};
use crate::rtweekend::{Point3, Ray, Vec3, PI};
use std::sync::Arc;

pub struct Sphere {
    pub centre: Point3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

//constructors

impl Sphere {
    pub fn new(c: Vec3, r: f32, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            centre: c,
            radius: r,
            material: mat,
        }
    }
}

//Spheres are hitables

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.centre;
        let a = r.direction.sq_len();
        let half_b = Vec3::dot(&oc, &r.direction);
        let c = oc.sq_len() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0f32 {
            return None;
        }

        let sqrtd = f32::sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let impact_point = r.at(root);
        let outward_normal = (1.0 / self.radius) * (impact_point - self.centre);
        let (a, b) = self.get_uv(&outward_normal);
        let mut hit_record = HitRecord {
            t: root,
            p: impact_point,
            normal: outward_normal,
            front_face: true,
            material: Arc::clone(&self.material),
            u: a,
            v: b,
        };
        hit_record.set_face_normal(r, &outward_normal);
        return Some(hit_record);
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(
            self.centre - Vec3::new(self.radius, self.radius, self.radius),
            self.centre + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

//surface coorindate functions
impl Sphere {
    fn get_uv(
        &self,
        p: &Point3, //p is a point on a sphere of radius one centered at the origin
    ) -> (f32, f32) //returns u,v where
                    //u is the angle (normalized) between x and z coordinates around the y axis taken from x = -1
                    //v is the angle (normalized) of the point's hight in spherical coordinates
    {
        let [x, y, z] = p.as_array();
        let theta = -y.acos();
        let phi = (-z).atan2(x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}
