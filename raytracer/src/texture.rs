// use std::sync::Arc;

// use crate::vec3::{Color, Point3};

// pub trait Texture {
//     fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
// }

// pub struct SolidColor {
//     color_value: Color,
// }

// impl SolidColor {
//     pub fn new(color_value: Color) -> Self {
//         SolidColor { color_value }
//     }

//     // pub fn gdb(red: f64, green: f64, blue: f64) -> Self {
//     //     SolidColor {
//     //         color_value: Color::new(red, green, blue),
//     //     }
//     // }
// }

// impl Texture for SolidColor {
//     fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
//         self.color_value
//     }
// }

// pub struct CheckerTexture {
//     even: Arc<dyn Texture>,
//     odd: Arc<dyn Texture>,
// }

// impl CheckerTexture {
//     pub fn new_from_color(c1: Color, c2: Color) -> Self {
//         Self {
//             even: Arc::new(SolidColor::new(c1)),
//             odd: Arc::new(SolidColor::new(c2)),
//         }
//     }

//     pub fn new(_even: Arc<dyn Texture>, _odd: Arc<dyn Texture>) -> Self {
//         Self {
//             even: _even,
//             odd: _odd,
//         }
//     }
// }

// impl Texture for CheckerTexture {
//     fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
//         let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
//         if sines < 0.0 {
//             self.odd.value(u, v, p)
//         } else {
//             self.even.value(u, v, p)
//         }
//     }
// }
