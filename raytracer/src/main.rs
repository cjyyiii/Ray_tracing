mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::vec3::Color;
use camera::Camera;
use color::write_color;
use hittable::Hittable;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
pub use ray::Ray;
use sphere::Sphere;
use std::fs::File;
use vec3::Point3;
pub use vec3::Vec3;

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
    let samples_per_pixel: u64 = 100;
    let max_depth = 50;
    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    let mut world: HittableList = HittableList::new();
    HittableList::add(
        &mut world,
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
    );
    HittableList::add(
        &mut world,
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    );

    let cam: Camera = Camera::new();

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar: ProgressBar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    for j in 0..height {
        for i in 0..width {
            let mut pixel_c: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u_rand: f64 = rng.gen();
                let v_rand: f64 = rng.gen();
                let u: f64 = (i as f64 + u_rand) / (width as f64 - 1.0);
                let v: f64 = (j as f64 + v_rand) / (height as f64 - 1.0);
                let r: Ray = cam.get_ray(u, v);
                pixel_c += ray_color(r, &world, max_depth);
            }
            let pixel_color: [u8; 3] = [
                (clamp((pixel_c.x() * 0.01).sqrt(), 0.0, 0.999) * 255.).floor() as u8,
                (clamp((pixel_c.y() * 0.01).sqrt(), 0.0, 0.999) * 255.).floor() as u8,
                (clamp((pixel_c.z() * 0.01).sqrt(), 0.0, 0.999) * 255.).floor() as u8,
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

fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.hit(&r, 0.0, f64::INFINITY) {
        let target: Point3 = hit_rec.p + hit_rec.normal + Vec3::random_in_unit_sphere();
        return 0.5 * ray_color(Ray::new(hit_rec.p, target - hit_rec.p), world, depth - 1);
    }

    let unit_direction: Vec3 = Vec3::unit_vector(r.dir);
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    let ray_col: Color = Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
    ray_col
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
