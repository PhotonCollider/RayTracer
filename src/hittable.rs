use std::sync::Arc;

use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::{Isotropic, Lambertian, Material};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::util::random_f64_0_1;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: true,
            mat: Arc::from(Lambertian::from_color(Vec3::ones())),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = r.b_direction * (*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABB;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bounding_box: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: vec![],
            bounding_box: AABB::default(),
        }
    }
    pub fn new_and_add(object: Arc<dyn Hittable>) -> Self {
        let mut ret = Self::new();
        ret.add(object);
        ret
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bounding_box = AABB::new_two_boxes(self.bounding_box, object.bounding_box());
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far = ray_t.max;

        // pub objects: Vec<Arc<dyn Hittable>>
        for object in self.objects.iter() {
            if object.hit(
                r,
                Interval::with_bounds(ray_t.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}

unsafe impl Send for HittableList {}
unsafe impl Sync for HittableList {}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bounding_box: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bounding_box = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bounding_box,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_range: Interval, rec: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r = Ray::new(r.a_origin - self.offset, r.b_direction, r.time);

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if !self.object.hit(&offset_r, t_range, rec) {
            return false;
        }

        // Move the intersection point forwards by the offset
        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    cos_theta: f64,
    sin_theta: f64,
    bounding_box: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let mut bounding_box = object.bounding_box();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = if i == 1 {
                        bounding_box.x.max
                    } else {
                        bounding_box.x.min
                    };
                    let y = if j == 1 {
                        bounding_box.y.max
                    } else {
                        bounding_box.y.min
                    };
                    let z = if k == 1 {
                        bounding_box.z.max
                    } else {
                        bounding_box.z.min
                    };

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        *min.mut_lp(c) = f64::min(min.lp(c), tester.lp(c));
                        *max.mut_lp(c) = f64::max(max.lp(c), tester.lp(c));
                    }
                }
            }
        }

        bounding_box = AABB::new_two_points(min, max);
        Self {
            object,
            cos_theta,
            sin_theta,
            bounding_box,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_range: Interval, rec: &mut HitRecord) -> bool {
        // Change the ray from world space to object space
        let mut origin = r.a_origin;
        let mut direction = r.b_direction;

        origin.x = self.cos_theta * r.a_origin.x - self.sin_theta * r.a_origin.z;
        origin.z = self.sin_theta * r.a_origin.x + self.cos_theta * r.a_origin.z;

        direction.x = self.cos_theta * r.b_direction.x - self.sin_theta * r.b_direction.z;
        direction.z = self.sin_theta * r.b_direction.x + self.cos_theta * r.b_direction.z;

        let rotated_r = Ray::new(origin, direction, r.time);

        // Determine whether an intersection exists in object space (and if so, where)
        if !self.object.hit(&rotated_r, t_range, rec) {
            return false;
        }

        // Change the intersection point from object space to world space
        let mut p = rec.p;
        p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

        // Change the normal from object space to world space
        let mut normal = rec.normal;
        normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
        normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Vec3) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::from(Isotropic::from_color(albedo)),
        }
    }
    pub fn from_tex(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::from(Isotropic::from_texture(tex)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.hit(r, Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(
            r,
            Interval::with_bounds(rec1.t + 0.0001, f64::INFINITY),
            &mut rec2,
        ) {
            return false;
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.b_direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_f64_0_1().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.normal = Vec3::new(1.0,0.0,0.0);  // arbitrary
        rec.front_face = true;     // also arbitrary
        rec.mat = self.phase_function.clone();

        true
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
