use crate::{perlin::Perlin, util::Vec3};
use opencv::imgcodecs::imread;
use opencv::{
    core::{MatTraitConst, VecN},
    imgcodecs::IMREAD_COLOR,
};
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

// SolidColor
pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn from_vec(albedo: Vec3) -> Self {
        Self { albedo }
    }
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            albedo: Vec3::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.albedo
    }
}

// CheckerTexture
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn from_texture(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
    pub fn from_color(scale: f64, even: Vec3, odd: Vec3) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::from(SolidColor::from_vec(even)),
            odd: Arc::from(SolidColor::from_vec(odd)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let xint = (self.inv_scale * p.x).floor() as i32;
        let yint = (self.inv_scale * p.y).floor() as i32;
        let zint = (self.inv_scale * p.z).floor() as i32;
        if (xint + yint + zint) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

// ImageTexture
pub struct ImageTexture {
    pub img_data: opencv::core::Mat,
    width: u32,
    height: u32,
}

// unsafe impl Send for Image {}
// unsafe impl Sync for Image {}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let img_data = imread(&("./texture/".to_owned() + filename), IMREAD_COLOR)
            .expect("Image reading error!");
        let width = img_data.cols() as u32;
        let height = img_data.rows() as u32;
        Self {
            img_data,
            width,
            height,
        }
    }
    pub fn get_color(&self, mut u: f64, mut v: f64) -> Vec3 {
        // println!("u: {}, v: {}", u, v);
        if u <= 0.0 {
            u = 0.001;
        }
        if u >= 1.0 {
            u = 0.999;
        }
        if v <= 0.0 {
            v = 0.001;
        }
        if v >= 1.0 {
            v = 0.999;
        }

        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let color: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();
        // println!("color: {:?}", color);

        Vec3::new(color[2] as f64, color[1] as f64, color[0] as f64) * (1.0 / 255.0)
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        if self.width == 0 || self.height == 0 {
            return Vec3::new(0.0, 1.0, 1.0);
        }
        let org_color = self.get_color(u, v);

        //Adjust the color to right gamma
        Vec3::new(
            org_color.x * org_color.x,
            org_color.y * org_color.y,
            org_color.z * org_color.z,
        )
    }
}

// NoiseTexture
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
        // Vec3::new(1.0, 1.0, 1.0) * self.noise.turb(p, 7)
    }
}
