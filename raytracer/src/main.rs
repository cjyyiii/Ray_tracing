mod color;
mod ray;
mod vec3;

use crate::vec3::Color;
use color::write_color;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use ray::Ray;
use std::{fs::File, mem::discriminant};
pub use vec3::Vec3;

use crate::vec3::Point3;

const AUTHOR: &str = "程婧祎";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci: bool = is_ci();

    println!("CI: {}", is_ci);

    let width: usize = 800;
    let height: usize = 800;
    let path: &str = "output/test.jpg";
    let quality: u8 = 60; // From 0 to 100, suggested value: 60

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = 1.0 * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Vec3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar: ProgressBar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in 0..height {
        for i in 0..width {
            let u: f64 = i as f64 / (width - 1) as f64;
            let v: f64 = j as f64 / (height - 1) as f64;
            let direction: Vec3 = lower_left_corner + u * horizontal + v * vertical - origin;
            let r: Ray = Ray::new(origin, direction);
            let pixel_color: [u8; 3] = [
                (ray_color(r).x() as f32 * 255.).floor() as u8,
                (ray_color(r).y() as f32 * 255.).floor() as u8,
                (ray_color(r).z() as f32 * 255.).floor() as u8,
            ];
            write_color(pixel_color, &mut img, i, height - j - 1);
            bar.inc(1);
        }
    }

    // Finish progress bar
    bar.finish();

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}

fn ray_color(r: Ray) -> Color {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        let ret: Color = Color::new(1.0, 0.0, 0.0);
        return ret;
    }
    //2
    // let t: f64 = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    // if t > 0.0 {
    //     let N: Vec3 = Vec3::unit_vector(Ray::at(&r, t) - Vec3::new(0.0, 0.0, -1.0));
    //     return 0.5 * Color::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    // }
    let unit_direction: Vec3 = Vec3::unit_vector(r.dir);
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    // let t = 0.5 * (unit_direction.y() + 1.0);
    let ray_col: Color = Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
    ray_col
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool {
    let oc: Vec3 = r.orig - center;
    let a: f64 = Vec3::dot(r.dir, r.dir);
    let b: f64 = 2.0 * Vec3::dot(oc, r.dir);
    let c: f64 = Vec3::dot(oc, oc) - radius * radius;
    let discriminant: f64 = b * b - 4.0 * a * c;
    let flag: bool = discriminant > 0.0;
    return flag;
    // 2
    // if discriminant < 0.0 {
    //     return -1.0;
    // } else {
    //     return (-b - discriminant.sqrt()) / (2.0 * a);
    // }
}
