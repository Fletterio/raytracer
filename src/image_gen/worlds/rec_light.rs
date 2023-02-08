use crate::{
    hitable::{
        hitable_list::HitableList,
        material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
        sphere::Sphere,
        xy_rect::XYRect,
    },
    texture::noise::NoiseTexture,
};

pub fn light() -> HitableList {
    let mut objects = HitableList::new(vec![]);

    let pertext = Arc::new(NoiseTexture::new(4));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Arc::clone(&pertext))),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(Arc::clone(&pertext))),
    )));

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    objects
}
