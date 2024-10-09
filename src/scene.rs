use std::sync::Arc;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::hittable::{ConstantMedium, HittableList, RotateY, Translate};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::quad::{box_from_vec, Quad};
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::util::{
    random_f64_0_1, random_f64_ranged, random_positive_vec3, random_positive_vec3_ranged,
};
use crate::vec3::Vec3;

pub fn bouncing_spheres() -> (Camera, HittableList) {
    // World
    let mut world = HittableList::new();

    let checker = Arc::from(CheckerTexture::from_color(
        0.32,
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::from(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::from(Lambertian::from_texture(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64_0_1();
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64_0_1(),
                0.2,
                b as f64 + 0.9 * random_f64_0_1(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_positive_vec3().component_mul(random_positive_vec3());
                    sphere_material = Arc::from(Lambertian::from_color(albedo));
                    let center2 = center + Vec3::new(0.0, random_f64_ranged(0.0, 0.5), 0.0);
                    world.add(Arc::from(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_positive_vec3_ranged(0.5, 1.0);
                    let fuzz = random_f64_ranged(0.0, 0.5);
                    sphere_material = Arc::from(Metal::new(albedo, fuzz));
                    world.add(Arc::from(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::from(Dielectric::new(1.5));
                    world.add(Arc::from(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::from(Dielectric::new(1.5));
    world.add(Arc::from(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::from(Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::from(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::from(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::from(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world = HittableList::new_and_add(Arc::from(BVHNode::new(world)));

    let mut cam = Camera::default();

    cam.image_width = 400;
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::zero();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    (cam, world)
}

pub fn checkered_spheres() -> (Camera, HittableList) {
    let mut world = HittableList::new();
    let checker = Arc::from(CheckerTexture::from_color(
        0.32,
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::from(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::from(Lambertian::from_texture(checker.clone())),
    )));
    world.add(Arc::from(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::from(Lambertian::from_texture(checker)),
    )));

    let mut cam = Camera::default();

    cam.image_width = 400;
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::zero();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    (cam, world)
}

pub fn earth() -> (Camera, HittableList) {
    let mut world = HittableList::new();
    let earth_texture = Arc::from(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::from(Lambertian::from_texture(earth_texture));

    world.add(Arc::from(Sphere::new(Vec3::zero(), 2.0, earth_surface)));

    let mut cam = Camera::default();

    cam.image_width = 400;
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(0.0, 0.0, 12.0);
    cam.lookat = Vec3::zero();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    (cam, world)
}

pub fn perlin_spheres() -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let mut cam = Camera::default();

    cam.image_width = 400;
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::zero();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    (cam, world)
}

pub fn quads() -> (Camera, HittableList) {
    // Materials
    let left_red = Arc::new(Lambertian::from_color(Vec3::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from_color(Vec3::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::from_color(Vec3::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::from_color(Vec3::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::from_color(Vec3::new(0.2, 0.8, 0.8)));

    // Quads
    let mut world = HittableList::new();
    world.add(Arc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal.clone(),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.70, 0.80, 1.00);

    cam.vfov = 80.0;
    cam.lookfrom = Vec3::new(0.0, 0.0, 9.0);
    cam.lookat = Vec3::zero();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    (cam, world)
}

pub fn simple_light() -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let difflight = Arc::new(DiffuseLight::from_color(Vec3::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vec3::zero();

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(26.0, 3.0, 6.0);
    cam.lookat = Vec3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    (cam, world)
}

pub fn cornell_box() -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Vec3::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::zero(),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::zero(),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = box_from_vec(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let box1 = Arc::from(RotateY::new(box1, 15.0));
    let box1 = Arc::from(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = box_from_vec(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), white.clone());
    let box2 = Arc::from(RotateY::new(box2, -18.0));
    let box2 = Arc::from(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let mut cam = Camera::default();
    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.sample_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Vec3::zero();

    cam.vfov = 40.0;
    cam.lookfrom = Vec3::new(278.0, 278.0, -800.0);
    cam.lookat = Vec3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    (cam, world)
}

pub fn cornell_smoke() -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let red = Arc::from(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::from(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::from(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::from(DiffuseLight::from_color(Vec3::new(7.0, 7.0, 7.0)));

    world.add(Arc::from(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::from(Quad::new(
        Vec3::zero(),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::from(Quad::new(
        Vec3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Arc::from(Quad::new(
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::from(Quad::new(
        Vec3::zero(),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::from(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = box_from_vec(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let box1 = Arc::from(RotateY::new(box1, 15.0));
    let box1 = Arc::from(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let box2 = box_from_vec(Vec3::zero(), Vec3::new(165.0, 165.0, 165.0), white.clone());
    let box2 = Arc::from(RotateY::new(box2, -18.0));
    let box2 = Arc::from(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Arc::from(ConstantMedium::from_color(
        box1,
        0.01,
        Vec3::zero(),
    )));
    world.add(Arc::from(ConstantMedium::from_color(
        box2,
        0.01,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.sample_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Vec3::zero();

    cam.vfov = 40.0;
    cam.lookfrom = Vec3::new(278.0, 278.0, -800.0);
    cam.lookat = Vec3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    (cam, world)
}

pub fn final_scene(
    image_width: u32,
    sample_per_pixel: u32,
    max_depth: u32,
) -> (Camera, HittableList) {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::from_color(Vec3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_ranged(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(box_from_vec(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();
    world.add(Arc::new(BVHNode::new(boxes1)));

    let light = Arc::new(DiffuseLight::from_color(Vec3::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Vec3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::from_color(Vec3::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Vec3::zero(),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::from_texture(Arc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    world.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::random_ranged(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BVHNode::new(boxes2)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = 1.0;
    cam.image_width = image_width;
    cam.sample_per_pixel = sample_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Vec3::zero();
    cam.vfov = 40.0;
    cam.lookfrom = Vec3::new(478.0, 278.0, -600.0);
    cam.lookat = Vec3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    (cam, world)
}

// this scene is just for fun XD
pub fn joe_fight(
    image_width: u32,
    sample_per_pixel: u32,
    max_depth: u32,
) -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    let joetext = Arc::new(ImageTexture::new("169joefight.png"));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(joetext)),
    )));

    let big_difflight = Arc::new(DiffuseLight::from_color(Vec3::new(7.0, 7.0, 7.0)));
    let small_difflight = Arc::new(DiffuseLight::from_color(Vec3::ones() / 2.0));
    world.add(Arc::new(Quad::new(
        Vec3::new(2.5, 0.5, -3.0),
        Vec3::new(3.0, 0.0, 0.0),
        Vec3::new(0.0, 3.0, 0.0),
        big_difflight.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 13.0, 0.0),
        7.0,
        small_difflight.clone(),
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = image_width;
    cam.sample_per_pixel = sample_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Vec3::zero();

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(26.0, 3.0, 6.0);
    cam.lookat = Vec3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    (cam, world)
}
