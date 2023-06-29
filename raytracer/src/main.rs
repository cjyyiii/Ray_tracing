mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod vec3;

use crate::material::{Dielectric, Lambertian, Metal};
use crate::vec3::{Color, Point3, Vec3};
use bvh::BVHNode;
use camera::Camera;
use color::write_color;
use hittable::Hittable;
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
pub use ray::Ray;
use sphere::{MovingSphere, Sphere};
use std::fs::File;
use std::sync::Arc;
use texture::{CheckerTexture, ImageTexture, NoiseTexture};

const AUTHOR: &str = "程婧祎";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();
    let material_ground: Lambertian = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    let checker = Arc::new(CheckerTexture::new_from_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new_arc(checker),
    )));

    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choosemat: f64 = rng.gen();
            let deltax: f64 = rng.gen();
            let deltaz: f64 = rng.gen();
            let center: Point3 = Point3::new(a as f64 + 0.9 * deltax, 0.2, b as f64 + 0.9 * deltaz);

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choosemat < 0.8 {
                    let r: f64 = rng.gen_range(0.0..1.0);
                    let g: f64 = rng.gen_range(0.0..1.0);
                    let b: f64 = rng.gen_range(0.0..1.0);
                    let p: Vec3 = Vec3::new(r, g, b);
                    let albedo: Vec3 = p * p;
                    let center2: Vec3 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choosemat < 0.95 {
                    let r: f64 = rng.gen_range(0.5..1.0);
                    let g: f64 = rng.gen_range(0.5..1.0);
                    let b: f64 = rng.gen_range(0.5..1.0);
                    let albedo: Vec3 = Vec3::new(r, g, b);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    HittableList::add(
                        &mut world,
                        Arc::new(Sphere::new(center, 0.2, sphere_material)),
                    );
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn two_speres() -> HittableList {
    let mut world: HittableList = HittableList::new();
    let checker = Arc::new(CheckerTexture::new_from_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let checker1 = checker.clone();
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new_arc(checker),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new_arc(checker1),
    )));
    world
}

fn two_perlin_spheres() -> HittableList {
    let mut world: HittableList = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new_sc(4.0));
    let pertext1 = Arc::new(NoiseTexture::new_sc(4.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new_arc(pertext),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new_arc(pertext1),
    )));
    world
}

fn earth_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Lambertian::new_arc(earth_texture);
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));
    world
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci: bool = is_ci();

    println!("CI: {}", is_ci);

    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: usize = 400;
    let height: usize = 225;
    let path: &str = "output/test.jpg";
    let quality: u8 = 60; // From 0 to 100, suggested value: 60
    let samples_per_pixel: u64 = 200;
    let max_depth = 50;
    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    let world_scene;

    let lookfrom;
    let lookat;
    let vfov;
    let mut aperture: f64 = 0.0;

    match 0 {
        1 => {
            world_scene = random_scene();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world_scene = two_speres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        3 => {
            world_scene = two_perlin_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        _ => {
            world_scene = earth_scene();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
    }

    let world: Arc<dyn Hittable> = BVHNode::new_boxed(world_scene, 0.0, 1.0);

    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

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
                pixel_c += ray_color(r, &*world, max_depth);
            }
            let pixel_color: [u8; 3] = [
                (clamp(
                    (pixel_c.x() * 1.0 / samples_per_pixel as f64).sqrt(),
                    0.0,
                    0.999,
                ) * 255.)
                    .floor() as u8,
                (clamp(
                    (pixel_c.y() * 1.0 / samples_per_pixel as f64).sqrt(),
                    0.0,
                    0.999,
                ) * 255.)
                    .floor() as u8,
                (clamp(
                    (pixel_c.z() * 1.0 / samples_per_pixel as f64).sqrt(),
                    0.0,
                    0.999,
                ) * 255.)
                    .floor() as u8,
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

    if let Some(hit_rec) = world.hit(&r, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = hit_rec.mat_ptr.scatter(&r, &hit_rec) {
            return attenuation * ray_color(scattered, world, depth - 1);
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction: Vec3 = Vec3::unit_vector(r.dir);
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    let ray_col: Color = Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
    ray_col
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
