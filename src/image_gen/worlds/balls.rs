use crate::hitable::{
    hitable_list::HitableList,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    moving_sphere::MovingSphere,
    sphere::Sphere,
    Hitable,
};
use crate::rtweekend::{random_double, random_double_between, Color, Point3, Vec3};
use crate::texture::{checker::CheckerTexture, SolidColor, Texture};
use std::sync::Arc;

pub fn random_scene() -> HitableList {
    let mut world = HitableList::new(vec![]);

    let checkerboard: Arc<CheckerTexture> = Arc::new(CheckerTexture::from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(checkerboard)),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f32 + 0.9 * random_double(),
                0.2,
                b as f32 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(Arc::new(SolidColor::from(albedo))));
                    let center2 = center + Vec3::new(0.0, random_double_between(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_between(0.5, 1.0);
                    let fuzz = random_double_between(0.0, 0.4);
                    sphere_material =
                        Arc::new(Metal::new(Arc::new(SolidColor::from(albedo)), fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    //glass
                    sphere_material = Arc::new(Dielectric::new(1.8));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let glass = Arc::new(Dielectric::new(1.7));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        glass,
    )));

    let lambert = Arc::new(Lambertian::new(Arc::new(SolidColor::from(Color::new(
        0.8, 0.1, 0.7,
    )))));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        lambert,
    )));

    let metal = Arc::new(Metal::new(
        Arc::new(SolidColor::from(Color::new(0.8, 0.8, 0.8))),
        0.0,
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        metal,
    )));

    world
}
