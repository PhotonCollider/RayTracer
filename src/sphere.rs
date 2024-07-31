use std::f64::consts::PI;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Arc<dyn Material>,
    velocity: Vec3,
    is_moving: bool,
    bounding_box: AABB,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
            velocity: Vec3::zero(),
            is_moving: false,
            bounding_box: AABB::new_two_points(
                center - Vec3::new(radius, radius, radius),
                center + Vec3::new(radius, radius, radius),
            ),
        }
    }

    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::new_two_points(center1 - rvec, center1 + rvec);
        let box2 = AABB::new_two_points(center2 - rvec, center2 + rvec);
        Self {
            center: center1,
            radius,
            mat,
            velocity: center2 - center1,
            is_moving: true,
            bounding_box: AABB::new_two_boxes(box1, box2),
        }
    }

    pub fn get_center(&self, time: f64) -> Vec3 {
        self.center + self.velocity * time
    }

    pub fn bounding_box(&self) -> AABB {
        self.bounding_box
    }

    pub fn get_sphere_uv(p: Vec3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // returns t in rec
        let center: Vec3 = self.get_center(r.time);
        let oc = center - r.a_origin;
        let a = r.b_direction.squared_length();
        let h = r.b_direction * oc;
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.mat = self.mat.clone();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);
        (rec.u, rec.v) = Sphere::get_sphere_uv(outward_normal);
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}
