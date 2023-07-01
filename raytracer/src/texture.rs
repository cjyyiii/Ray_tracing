use image::ImageBuffer;
pub use std::path::*;
use std::sync::Arc;

use crate::{
    clamp,
    perlin::Perlin,
    vec3::{Color, Point3},
};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        SolidColor { color_value }
    }

    // pub fn gdb(red: f64, green: f64, blue: f64) -> Self {
    //     SolidColor {
    //         color_value: Color::new(red, green, blue),
    //     }
    // }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new_from_color(c1: Color, c2: Color) -> Self {
        Self {
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }

    // pub fn new(_even: Arc<dyn Texture>, _odd: Arc<dyn Texture>) -> Self {
    //     Self {
    //         even: _even,
    //         odd: _odd,
    //     }
    // }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    // pub fn new() -> Self {
    //     NoiseTexture {
    //         noise: Perlin::new(),
    //         scale: 0.0,
    //     }
    // }

    pub fn new_sc(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (10.0 * self.noise.turb(p, 7) + p.z() * self.scale).sin())
    }
}

pub struct ImageTexture {
    pub data: ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
    pub width: usize,
    pub height: usize,
    pub bytes_per_scanline: usize,
}

impl ImageTexture {
    //图片读取基于https://github.com/Junxix/raytracer-2021/blob/master/raytracer/src/texture.rs
    const BYTES_PER_PIXEL: usize = 3;

    pub fn new(filename: &str) -> Self {
        // let components_per_pixel = ImageTexture::BYTES_PER_PIXEL;

        let data = image::open(filename).unwrap().to_rgb8();
        // let width = data.width() as usize;
        // let height = data.height() as usize;
        let (width, height) = data.dimensions();
        let bytes_per_scanline = ImageTexture::BYTES_PER_PIXEL * width as usize;

        Self {
            data,
            width: width as usize,
            height: height as usize,
            bytes_per_scanline,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Point3) -> Color {
        if self.data.is_empty() {
            Color::new(0.0, 1.0, 1.0)
        } else {
            let _u = clamp(u, 0.0, 1.0);
            let _v = clamp(v.abs(), 0.0, 1.0);

            let mut i = (_u * self.width as f64).floor() as usize;
            let mut j = (_v * self.height as f64).floor() as usize;
            if i >= self.width {
                i = self.width - 1;
            }
            if j >= self.height {
                j = self.height - 1;
            }

            const COLOR_SCALE: f64 = 1.0 / 255.0;
            let pixel = self.data.get_pixel(i as u32, j as u32);
            let [r, g, b] = pixel.0;
            // println!("{};{};{}", r, g, b);
            Color::new(
                COLOR_SCALE * (r as f64),
                COLOR_SCALE * (g as f64),
                COLOR_SCALE * (b as f64),
            )
        }
    }
}
