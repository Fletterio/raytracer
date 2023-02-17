use std::{fs::File, sync::Arc};

use crate::{
    bvh_node::BVHNode,
    hitable::{
        hitable_list::HitableList,
        material::{
            dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian,
            metal::Metal, Material,
        },
        moving_sphere::MovingSphere,
        rectangles::XZRect,
        rotation::RotateY,
        solid_box::SolidBox,
        sphere::Sphere,
        translate::Translate,
        volume::constant_medium::ConstantMedium,
        Hitable,
    },
    rtweekend::random_double_between,
    texture::{image::ImageTexture, noise::NoiseTexture, SolidColor},
    vec3::{Color, Point3, Vec3},
};

pub fn scene() -> HitableList {
    let ground: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side: usize = 20;
    let mut boxes1: Vec<Arc<dyn Hitable>> = Vec::with_capacity(boxes_per_side * boxes_per_side);
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_between(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.push(Arc::new(SolidBox::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground),
            )));
        }
    }

    let mut objects = HitableList::new(vec![]);

    objects.add(Arc::new(BVHNode::new(&boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::from_color(Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 43.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Arc::new(SolidColor::new(0.0, 0.8, 0.9)), 1.0)),
    )));

    let boundary: Arc<dyn Hitable> = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::clone(&boundary));
    objects.add(Arc::new(ConstantMedium::from_color(
        Arc::clone(&boundary),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary2 = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::from_color(
        boundary2,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let earth_path = File::open("textures/earthmap.png").unwrap();
    let emat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
        &earth_path,
        image::ImageFormat::Png,
    ))));
    objects.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    let mut boxes2: Vec<Arc<dyn Hitable>> = vec![];
    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for j in 0..ns {
        boxes2.push(Arc::new(Sphere::new(
            Point3::random_between(0.0, 165.0),
            10.0,
            Arc::clone(&white),
        )));
    }
    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVHNode::new(&boxes2, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}
