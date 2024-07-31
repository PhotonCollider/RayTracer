use crate::util::{fmax, fmin, random_f64_ranged};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// by Senpai
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul for Vec3 {
    //相当于重载了 *，重载的是点乘（内积）
    //用法： a = Vec3::new(1.0, 2.0, 3.0),b = Vec3::new(2.0, 3.0, 4.0),c:f64 = a * b = 20
    type Output = f64;
    fn mul(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    //重载了标量除法 / 。
    ///用法： a = Vec3::new(1.0, 2.0, 3.0)，b:f64  = 2.0, c:Vec3 = a / b = (0.5,1.0,1.5)
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
impl Vec3 {
    //取 x,y,z。理论上x,y,z是pub可以直接取，这样显得专业(TAT)
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn lp(&self, index: u8) -> f64 {
        if index == 0 {
            self.x
        } else if index == 1 {
            self.y
        } else {
            self.z
        }
    }

    //忘了是什么的缩写了，用来在循环中遍历向量
    //用法：a:Vec3 = Vec3::new(1.0,2.0,3.0) a.lp(0) = 1.0,a.lp(1) = 2.0,a.lp(2) = 3.0
    pub fn mut_lp(&mut self, index: u8) -> &mut f64 {
        if index == 0 {
            &mut self.x
        } else if index == 1 {
            &mut self.y
        } else {
            &mut self.z
        }
    }

    pub fn component_mul(&self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }

    //new 新建向量
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    //取两个向量中最小的作为向量，用于AABB(book 2)
    //用法：a = Vec3::new(1.0,2.0,3.0),b = Vec3::new(0.0,3.0,2.0)
    //Vec3::merge_min(&a,&b).info() -> x = 0 ,y = 2, z = 2
    //merge_max 类似
    pub fn merge_min(v1: &Vec3, v2: &Vec3) -> Self {
        Self {
            x: fmin(v1.x, v2.x),
            y: fmin(v1.y, v2.y),
            z: fmin(v1.z, v2.z),
        }
    }
    pub fn merge_max(v1: &Vec3, v2: &Vec3) -> Self {
        Self {
            x: (fmax(v1.x, v2.x)),
            y: fmax(v1.y, v2.y),
            z: fmax(v1.z, v2.z),
        }
    }

    // 新建全1向量
    //用法： let a = Vec3::ones();
    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
    // 新建全0向量
    //用法： let a = Vec3::zero();
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    //判断一个向量是否为0
    //由于解方程得到的值可能不严格等于0，所以用该方法判断0向量
    //if a.near_zero(){ ... }
    pub fn near_zero(&self) -> bool {
        self.x() < 0.00000001
            && self.x() > -0.00000001
            && self.y() < 0.00000001
            && self.y() > -0.00000001
            && self.z() < 0.00000001
            && self.z() > -0.00000001
    }

    //平方长度
    //a = Vec3::new(1.0,2.0,3.0),a.squared_length() = 1 + 4 + 9 = 14
    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    //模长
    //a = Vec3::new(1.0,2.0,3.0),a.length() = 1 + 4 + 9 = f64::sqrt(14) = ...
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    //叉乘
    //a = Vec3::new(1.0,2.0,3.0)，b = Vec3::new(1.0,2.0,3.0), (a.cross(b) == Vec3::zero()) = True
    pub fn cross(&self, other: Vec3) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vec3 {
            x: (x),
            y: (y),
            z: (z),
        }
    }
    // 用于调试信息，输出向量的内容
    //用法： a = Vec3::new(1.0,2.0,3.0),a.info()
    pub fn info(&self) {
        println!("x={},y={},z={}", self.x, self.y, self.z);
    }

    pub fn random_ranged(x: f64, y: f64) -> Self {
        Self {
            x: random_f64_ranged(x, y),
            y: random_f64_ranged(x, y),
            z: random_f64_ranged(x, y),
        }
    }
}

impl Add for Vec3 {
    //重载了向量加法
    //a = Vec3::new(1.0, 2.0, 3.0),b = Vec3::new(2.0, 3.0, 4.0),c:Vec3 = a + b == Vec3::new(3.0,5.0,7.0)
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    //重载了标量加法,同标量乘法，f64只能放在右边
    //a = Vec3::new(1.0, 2.0, 3.0),b:f64 = 2,c:Vec3 = a + b == Vec3::new(3.0,4.0,5.0)
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    //重载了向量加等于 +=,
    //let mut a = Vec3::new(1.0, 2.0, 3.0),b = Vec3::new(2.0, 3.0, 4.0),a += b
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}
impl Sub for Vec3 {
    //重载了向量减法
    //a = Vec3::new(1.0, 2.0, 3.0),b = Vec3::new(2.0, 3.0, 4.0),c:Vec3 = a - b == Vec3::new(-1.0,-1.0,-1.0)
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
