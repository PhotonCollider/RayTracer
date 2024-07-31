use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
};

pub struct BVHNode {
    bounding_box: AABB,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl BVHNode {
    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: i32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap()
    }

    pub fn new(mut list: HittableList) -> Self {
        let length = list.objects.len();
        BVHNode::init_from_list(list.objects.as_mut(), length)
    }
    fn init_from_list(vec: &mut [Arc<dyn Hittable>], object_span: usize) -> Self {
        let mut bounding_box = AABB::EMPTY;
        for i in 0..object_span {
            bounding_box = bounding_box.union(vec[i].bounding_box());
        }

        let axis = bounding_box.longest_axis();

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        if object_span == 1 {
            left = vec[0].clone();
            right = vec[0].clone();
        } else if object_span == 2 {
            left = vec[0].clone();
            right = vec[1].clone();
        } else {
            vec[..object_span].sort_by(|a, b| Self::box_compare(a, b, axis));

            let mid = object_span / 2;
            left = Arc::from(BVHNode::init_from_list(vec[..mid].as_mut(), mid));
            right = Arc::from(BVHNode::init_from_list(
                vec[mid..object_span].as_mut(),
                object_span - mid,
            ));
        }

        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            Interval::with_bounds(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );
        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}
