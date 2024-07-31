use crate::{
    interval::Interval,
    util::{Ray, Vec3},
};

#[derive(Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn default() -> Self {
        Self {
            x: Interval::new(),
            y: Interval::new(),
            z: Interval::new(),
        }
    }
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut ret = Self { x, y, z };
        ret.pad_to_minimums();
        ret
    }
    pub fn new_two_points(a: Vec3, b: Vec3) -> Self {
        let mut ret = Self {
            x: Interval::with_orderless_bounds(a.x, b.x),
            y: Interval::with_orderless_bounds(a.y, b.y),
            z: Interval::with_orderless_bounds(a.z, b.z),
        };
        ret.pad_to_minimums();
        ret
    }
    pub fn new_two_boxes(a: AABB, b: AABB) -> Self {
        let mut ret = Self {
            x: a.x.union(b.x),
            y: a.y.union(b.y),
            z: a.z.union(b.z),
        };
        ret.pad_to_minimums();
        ret
    }
    fn pad_to_minimums(&mut self) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
    pub fn union(&self, rhs: AABB) -> Self {
        Self {
            x: self.x.union(rhs.x),
            y: self.y.union(rhs.y),
            z: self.z.union(rhs.z),
        }
    }
    pub fn axis_interval(&self, id: i32) -> Interval {
        if id == 0 {
            self.x
        } else if id == 1 {
            self.y
        } else {
            self.z
        }
    }
    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else {
            if self.y.size() > self.z.size() {
                1
            } else {
                2
            }
        }
    }
    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig: &Vec3 = &r.a_origin;
        let ray_dir: &Vec3 = &r.b_direction;

        for axis in 0..3 {
            let ax: Interval = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir.lp(axis as u8);

            let t0 = (ax.min - ray_orig.lp(axis as u8)) * adinv;
            let t1 = (ax.max - ray_orig.lp(axis as u8)) * adinv;

            ray_t = ray_t.intersect(Interval::with_orderless_bounds(t0, t1));

            if ray_t.max <= ray_t.min {
                if self.x.size() <= 0.0 || self.y.size() <= 0.0 || self.z.size() <= 0.0 {
                    println!("tryed to hit empty AABB!!!");
                    std::process::exit(0);
                }
                return false;
            }
        }
        true
    }

    pub const EMPTY: AABB = AABB {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: AABB = AABB {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };
}

impl std::ops::Add<Vec3> for AABB {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}