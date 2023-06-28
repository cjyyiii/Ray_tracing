// use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::ray::Ray;
// use crate::texture::{SolidColor, Texture};
use crate::vec3::Color;
use crate::vec3::Vec3;
use rand::Rng;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    // pub albedo: Arc<dyn Texture>,
    pub albedo: Color,
}

impl Lambertian {
    // pub fn new_arc(a: Arc<dyn Texture>) -> Self {
    //     Self { albedo: a }
    // }

    pub fn new(a: Color) -> Self {
        Self {
            // albedo: Arc::new(SolidColor::new(a)),
            albedo: Color::new(a.x(), a.y(), a.z()),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        if Vec3::near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }
        let attenuation: Color = self.albedo;
        // let attenuation: Color = self.albedo.value(rec.u, rec.v, &rec.p);
        let scattered: Ray = Ray::new(rec.p, scatter_direction, r_in.tm);
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, fuzz: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected: Vec3 = Vec3::reflect(Vec3::unit_vector(r_in.dir), rec.normal);
        let scattered: Ray = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.tm,
        );
        let attenuation: Color = self.albedo;
        if Vec3::dot(scattered.dir, rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation: Color = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64 = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction: Vec3 = Vec3::unit_vector(r_in.dir);
        let cos_theta: f64 = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3 = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen()
        {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        let scattered: Ray = Ray::new(rec.p, direction, r_in.tm);
        Some((scattered, attenuation))
    }
}
