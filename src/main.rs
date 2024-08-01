mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod scene;
mod sphere;
mod texture;
mod util;
mod vec3;

use std::fs::File;
use scene::final_scene;

const AUTHOR: &str = "PhotonCollider";

fn main() {
    let now = std::time::Instant::now();
    let path = "output/final_scene.png";

    // 10k spp
    // 800 10k 40
    let (mut cam, world) = final_scene(800, 10000, 40);
    cam.enable_ssaa = true;
    cam.part_num_x = 40;
    cam.part_num_y = 40;
    let img = cam.render(&world);

    println!("Output image as \"{}\"\nAuthor: {}", path, AUTHOR);

    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Png) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }

    println!("Total time cost: {}", now.elapsed().as_secs_f64());
}
