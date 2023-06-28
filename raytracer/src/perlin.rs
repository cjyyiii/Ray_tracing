// use crate::texture::Texture;
use crate::vec3::Point3;
use rand::Rng;

pub struct Perlin {
    pub ranfloat: Vec<f64>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
}

impl Perlin {
    const POINT_COUNT: i32 = 256;

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut ranfloat = Vec::new();
        for _ in 0..Perlin::POINT_COUNT {
            ranfloat.push(rng.gen_range(0.0..1.0));
        }

        Perlin {
            ranfloat,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        // let i = ((4.0 * p.x()) as i32) & 255;
        // let j = ((4.0 * p.y()) as i32) & 255;
        // let k = ((4.0 * p.z()) as i32) & 255;

        // self.ranfloat
        //     [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for (di, value1) in c.clone().iter().enumerate().take(2) {
            for (dj, value2) in value1.iter().enumerate().take(2) {
                for (dk, _) in value2.iter().enumerate().take(2) {
                    let _i = (i + di as i32) & 255;
                    let _j = (j + dj as i32) & 255;
                    let _k = (k + dk as i32) & 255;

                    c[di][dj][dk] = self.ranfloat[(self.perm_x[_i as usize]
                        ^ self.perm_y[_j as usize]
                        ^ self.perm_z[_k as usize])
                        as usize];
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        for (i, value1) in c.iter().enumerate().take(2) {
            for (j, value2) in value1.iter().enumerate().take(2) {
                for (k, _) in value2.iter().enumerate().take(2) {
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * c[i][j][k];
                }
            }
        }
        accum
    }

    pub fn perlin_generate_perm() -> Vec<i32> {
        let mut _p = Vec::new();

        for i in 0..Perlin::POINT_COUNT {
            _p.push(i);
        }

        Perlin::permute(&mut _p, Self::POINT_COUNT);

        _p
    }

    fn permute(p: &mut [i32], n: i32) {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        for i in (1..n).rev() {
            let target: i32 = rng.gen_range(0..i);
            p.swap(i as usize, target as usize);
            // let tmp = p[i as usize];
            // p[i as usize] = p[target as usize];
            // p[target as usize] = tmp;
        }
    }
}
