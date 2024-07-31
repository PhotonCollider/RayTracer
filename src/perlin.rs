use crate::util::{random_f64_0_1, random_i32_ranged, random_in_unit_sphere, Vec3};

const point_count: usize = 256;

pub struct Perlin {
    randvec: [Vec3; point_count],
    perm_x: [i32; point_count],
    perm_y: [i32; point_count],
    perm_z: [i32; point_count],
}

impl Perlin {
    pub fn new() -> Self {
        let mut ret = Self {
            randvec: [Vec3::zero(); point_count],
            perm_x: [0; point_count],
            perm_y: [0; point_count],
            perm_z: [0; point_count],
        };
        for i in 0..point_count {
            ret.randvec[i] = random_in_unit_sphere().unit();
        }

        Self::perlin_generate_perm(&mut ret.perm_x);
        Self::perlin_generate_perm(&mut ret.perm_y);
        Self::perlin_generate_perm(&mut ret.perm_z);
        ret
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interpolate(&c, u, v, w)

        /*
        let i = (4.0 * p.x) as i32 & 255;
        let j = (4.0 * p.y) as i32 & 255;
        let k = (4.0 * p.z) as i32 & 255;

        self.randfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
        */
    }

    pub fn turb(&self, p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
    
        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
    
        accum.abs()
    }

    fn trilinear_interpolate(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i][j][k]
                        * weight_v;
                }
            }
        }
        accum
    }

    fn perlin_generate_perm(p: &mut [i32; point_count]) {
        for i in 0..point_count {
            p[i] = i as i32;
        }
        Self::permute(p, point_count);
    }

    fn permute(p: &mut [i32; point_count], n: usize) {
        for i in (1..n).rev() {
            let target = random_i32_ranged(0, i as i32) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
}
