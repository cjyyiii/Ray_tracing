use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Sphere<M: Material> {
    pub center: Point3,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.orig - self.center;
        let a: f64 = r.dir.squared_length();
        let half_b: f64 = Vec3::dot(oc, r.dir);
        let c: f64 = Vec3::squared_length(&oc) - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd: f64 = discriminant.sqrt();

        let mut root: f64 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t: f64 = root;
        let p: Vec3 = r.at(t);
        let outward_normal: Vec3 = (p - self.center) / self.radius;

        let hit_rec: HitRecord = HitRecord::new(p, t, &self.material, outward_normal, *r);
        Some(hit_rec)
    }
}
