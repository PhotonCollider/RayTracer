use std::sync::Arc;

use crate::{
    hittable::HitRecord,
    texture::{SolidColor, Texture},
    util::{random_in_unit_sphere, reflect, refract, Ray, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::zero()
    }
}

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn from_color(albedo: Vec3) -> Self {
        Self {
            tex: Arc::from(SolidColor::from_vec(albedo)),
        }
    }
    pub fn from_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + random_in_unit_sphere().unit();
        *scattered = Ray::new(rec.p, scatter_direction, r_in.time);
        *attenuation = self.tex.value(rec.u, rec.v, rec.p);
        true
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(r_in.b_direction, rec.normal);
        reflected = reflected.unit() + random_in_unit_sphere().unit() * self.fuzz;
        *scattered = Ray::new(rec.p, reflected, r_in.time);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::ones();
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let refracted: Vec3 = refract(r_in.b_direction.unit(), rec.normal, ri);
        *scattered = Ray::new(rec.p, refracted, r_in.time);
        true
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_texture(tex: Arc<dyn Texture>) -> Self {
        DiffuseLight { tex }
    }

    pub fn from_color(emit: Vec3) -> Self {
        DiffuseLight {
            tex: Arc::new(SolidColor::from_vec(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.tex.value(u, v, p)
    }
}

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn from_color(albedo: Vec3) -> Self {
        Self {
            tex: Arc::from(SolidColor::from_vec(albedo)),
        }
    }
    pub fn from_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, random_in_unit_sphere().unit(), r_in.time);
        *attenuation = self.tex.value(rec.u, rec.v, rec.p);
        return true;
    }
}
