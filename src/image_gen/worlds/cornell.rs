use std::sync::Arc;

use crate::{
    hitable::{
        hitable_list::HitableList,
        material::{diffuse_light::DiffuseLight, lambertian::Lambertian, Material},
        solid_box::SolidBox,
    },
    vec3::Color,
};

use crate::hitable::rectangles::*;

pub fn cornell_box() -> HitableList {
    let mut objects = HitableList::new(vec![]);

    let red = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));

    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));

    let green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));

    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    //side walls
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    //light
    objects.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    //floor, ceiling and back wall
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));

    //boxes
    objects.add(Arc::new(SolidBox::new(
        Point3::new(130.0, 0.0, 65.0),
        Point3::new(295.0, 165.0, 230.0),
        Arc::clone(&white),
    )));
    objects.add(Arc::new(SolidBox::new(
        Point3::new(265.0, 0.0, 295.0),
        Point3::new(430.0, 330.0, 460.0),
        Arc::clone(&white),
    )));

    objects
}
