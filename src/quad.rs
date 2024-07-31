use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    material::Material,
    util::{Ray, Vec3},
};

// quadrilateral
#[derive(Clone)]
pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bounding_box: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let mut quad = Self {
            q,
            u,
            v,
            w: Vec3::zero(),
            mat,
            bounding_box: AABB::default(),
            normal: Vec3::zero(),
            d: 0.0,
        };
        let n = u.cross(v);
        quad.normal = n.unit();
        quad.d = quad.normal * quad.q;
        quad.w = n / (n * n); // this is n, not normal
        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        // Compute the bounding box of all four vertices.
        let bbox_diagonal1 = AABB::new_two_points(self.q, self.q + self.u + self.v);
        let bbox_diagonal2 = AABB::new_two_points(self.q + self.u, self.q + self.v);
        self.bounding_box = AABB::new_two_boxes(bbox_diagonal1, bbox_diagonal2);
    }

    fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::with_bounds(0.0, 1.0);
    
        // Given the hit Vec in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }
    
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = r.b_direction * self.normal;

        // No hit if the ray is parallel to the plane.
        if (denom.abs()) < 1e-8 {
            return false;
        }

        // Return false if the hit Vec parameter t is outside the ray interval.
        let t = (self.d - r.a_origin * self.normal) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine if the hit Vec lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitpt_Vector = intersection - self.q;
        let alpha = self.w * planar_hitpt_Vector.cross(self.v);
        let beta = self.w * self.u.cross(planar_hitpt_Vector);

        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}

pub fn box_from_vec(a: Vec3, b: Vec3, mat: Arc<dyn Material>) -> Arc<HittableList> {
    // Returns the 3D box (six sides) that contains the two opposite vertices a & b.
    let mut sides = HittableList::new();

    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = Vec3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Vec3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x(), min.y(), max.z()), dx, dy, mat.clone(),
    ))); // front
    sides.add(Arc::new(Quad::new(
        Vec3::new(max.x(), min.y(), max.z()), -dz, dy, mat.clone(),
    ))); // right
    sides.add(Arc::new(Quad::new(
        Vec3::new(max.x(), min.y(), min.z()), -dx, dy, mat.clone(),
    ))); // back
    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x(), min.y(), min.z()), dz, dy, mat.clone(),
    ))); // left
    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x(), max.y(), max.z()), dx, -dz, mat.clone(),
    ))); // top
    sides.add(Arc::new(Quad::new(
        Vec3::new(min.x(), min.y(), min.z()), dx, dz, mat,
    ))); // bottom

    Arc::new(sides)
}