// use crate::texture::Texture;
use crate::vec3::{Point3, Vec3};
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
        let i = ((4.0 * p.x()) as i32) & 255;
        let j = ((4.0 * p.y()) as i32) & 255;
        let k = ((4.0 * p.z()) as i32) & 255;

        self.ranfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
        // let u = p.x() - p.x().floor();
        // let v = p.y() - p.y().floor();
        // let w = p.z() - p.z().floor();

        // let i = p.x().floor() as i32;
        // let j = p.y().floor() as i32;
        // let k = p.z().floor() as i32;

        // let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

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
