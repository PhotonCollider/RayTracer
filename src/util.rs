use rand::rngs::ThreadRng;
use rand::Rng;

// pub use crate::aabb::BvhNode;
pub use crate::ray::Ray;
// pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;
// pub use crate::world::Object;
// use rand::{rngs::ThreadRng, Rng};

//计算单位向量
pub fn unit_vec(v: Vec3) -> Vec3 {
    v / v.length()
}
//自己实现的绝对值
pub fn fabs(num: f64) -> f64 {
    if num < 0.0 {
        -num
    } else {
        num
    }
}
//自己实现的取两数最小值
pub fn fmin(v1: f64, v2: f64) -> f64 {
    if v1 > v2 {
        v2
    } else {
        v1
    }
}
//自己实现的取两数最大值
pub fn fmax(v1: f64, v2: f64) -> f64 {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}

//折射模块，计算的是反射比，ratio为折射率之比
pub fn reflectance(cos_theta: f64, ratio: f64) -> f64 {
    let mut r0 = (1.0 - ratio) / (1.0 + ratio);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powf(1.0 - cos_theta, 5.0)
}

//ratio is etia / etia prime i.e the sphere is under the fraction
//计算折射光线，v为入射光线，n为法线，ratio为折射率之比
pub fn refract(v: Vec3, n: Vec3, ratio: f64) -> Vec3 {
    //v,n为单位向量
    //按道理应该不会有cos比1大
    let cos_theta = -v * n;
    let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
    let mut random: ThreadRng = rand::thread_rng();
    if ratio * sin_theta >= 1.0 || reflectance(cos_theta, ratio) > random.gen::<f64>() {
        // total reflectance
        reflect(v, n)
    } else {
        let perp = (v + n * cos_theta) * ratio;
        let para = -n * f64::sqrt(fabs(1.0 - perp.squared_length()));
        perp + para
    }
}

//反射模块，简单，v为入射光线，n为法线
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    // n为单位向量
    v - n * (v * n) * 2.0
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let v = random_in_unit_sphere();
    if normal * v >= 0.0 {
        v
    } else {
        -v
    }
}

//计算单位球中一个随机单位向量
pub fn random_in_unit_sphere() -> Vec3 {
    let mut random: ThreadRng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            random.gen_range(-1.0..1.0),
            random.gen_range(-1.0..1.0),
            random.gen_range(-1.0..1.0),
        );
        if p.squared_length() >= 1.0 {
            continue;
        }
        let tmp: Vec3 = unit_vec(p);
        if tmp.near_zero() {
            return Vec3::zero();
        } else {
            return tmp;
        }
    }
}

pub fn random_positive_vec3() -> Vec3 {
    let mut random: ThreadRng = rand::thread_rng();
    Vec3::new(
        random.gen_range(0.0..1.0),
        random.gen_range(0.0..1.0),
        random.gen_range(0.0..1.0),
    )
}

pub fn random_positive_vec3_ranged(x: f64, y: f64) -> Vec3 {
    let mut random: ThreadRng = rand::thread_rng();
    Vec3::new(
        random.gen_range(x..y),
        random.gen_range(x..y),
        random.gen_range(x..y),
    )
}

//0-1中随机数字
pub fn random_f64_0_1() -> f64 {
    let mut random: ThreadRng = rand::thread_rng();
    random.gen::<f64>()
}

pub fn random_f64_ranged(x: f64, y: f64) -> f64 {
    let mut random: ThreadRng = rand::thread_rng();
    random.gen_range(x..y)
}

// including x and y !!!
pub fn random_i32_ranged(x: i32, y: i32) -> i32 {
    random_f64_ranged(x as f64, (y + 1) as f64) as i32
}

//1-100随机数字
pub fn random_f64_101() -> f64 {
    let mut random: ThreadRng = rand::thread_rng();
    random.gen_range(1.0..100.0)
}

//0-165随机向量，用于生成随机的场景数据
pub fn random_cen_165() -> Vec3 {
    let mut random: ThreadRng = rand::thread_rng();
    Vec3::new(
        random.gen_range(0.0..165.0),
        random.gen_range(0.0..165.0),
        random.gen_range(0.0..165.0),
    )
}

//单位圆盘中随机向量
pub fn random_in_unit_disk() -> Vec3 {
    let mut random: ThreadRng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            random.gen_range(-1.0..1.0),
            random.gen_range(-1.0..1.0),
            0.0,
        );
        if p.squared_length() >= 1.0 {
            continue;
        }
        //let tmp = unit_vec(p);
        if p.near_zero() {
            return Vec3::zero();
        } else {
            return p;
        }
    }
}

//0-1截断函数
pub fn cut(x: f64) -> f64 {
    if x > 0.99 {
        0.99
    } else if x < 0.0 {
        0.0
    } else {
        x
    }
}

/*

//处理最近的光线交点(bvh版)
//球版只留下了注释的一点点
pub fn hittable(r: Ray, bvh_tree: &BvhNode) -> (f64, Object) {
    //let mut t = f64::INFINITY;
    let t_min = 0.001;
    //let mut sphere = &Sphere::empty_sphere();
    let (t, obj) = bvh_tree.hit(&r, t_min, f64::INFINITY);
    (t, obj)
}
// for i in v {
//     let (tmp, delta) = i.hit_sphere(r.clone());
//     if tmp < t_min || tmp > t {
//         let tmp = tmp + delta;
//         //可能影响折射
//         if tmp < t_min || tmp > t {
//             continue;
//         } else {
//             t = tmp;
//             sphere = i;
//         }
//     } else {
//         t = tmp;
//         sphere = i;
//     }
// }


*/
