use crate::interval::Interval;
use crate::vec3::Vec3;
use image::RgbImage;

/// the multi-sample write_color() function
// pub fn write_color(pixel_color: [u8; 3], img: &mut RgbImage, i: usize, j: usize) {
//     let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
//     *pixel = image::Rgb(pixel_color);
//     // Write the translated [0,255] value of each color component.
// }

/// the multi-sample write_color() function
pub fn write_color(pixel_color: Vec3, img: &mut RgbImage, i: usize, j: usize) {
    let interval: Interval = Interval::with_bounds(0.0, 255.0);
    let pixel = img.get_pixel_mut(i as u32, j as u32);
    *pixel = image::Rgb([
        interval.clamp(pixel_color.x.sqrt() * 256.0) as u8,
        interval.clamp(pixel_color.y.sqrt() * 256.0) as u8,
        interval.clamp(pixel_color.z.sqrt() * 256.0) as u8,
    ]);
    // Write the translated [0,255] value of each color component.
}
