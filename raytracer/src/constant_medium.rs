use std::{f64, sync::Arc};

// use console::Color;

use rand::Rng;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    texture::{SolidColor, Texture},
    vec3::Vec3,
    Ray,
};

pub struct ConstantMediun {
    pub boundary: Arc<dyn Hittable + Send + Sync>,
    pub phase_function: Arc<dyn Material + Send + Sync>,
    pub neg_inv_density: f64,
}

impl ConstantMediun {
    // pub fn new(b: Arc<dyn Hittable + Send + Sync>, d: f64, a: Arc<dyn Texture + Send + Sync>) -> Self {
    //     Self {
    //         boundary: b,
    //         phase_function: Arc::new(Iostropic::new(a)),
    //         neg_inv_density: -1.0 / d,
    //     }
    // }

    pub fn new_col(b: Arc<dyn Hittable + Send + Sync>, d: f64, c: Vec3) -> Self {
        Self {
            boundary: b,
            phase_function: Arc::new(Iostropic::new_col(c)),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl Hittable for ConstantMediun {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::Aabb> {
        self.boundary.bounding_box(time0, time1)
    }

    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        let rbg = rand::thread_rng().gen_range(0.0..1.0);
        if let Some(mut hit_rec1) = self.boundary.hit(r, -f64::INFINITY, f64::INFINITY) {
            if let Some(mut hit_rec2) = self.boundary.hit(r, hit_rec1.t + 0.0001, f64::INFINITY) {
                if hit_rec1.t < t_min {
                    hit_rec1 = HitRecord::new(
                        hit_rec1.p,
                        t_min,
                        hit_rec1.u,
                        hit_rec1.v,
                        hit_rec1.mat_ptr,
                        hit_rec1.normal,
                        *r,
                    );
                }
                if hit_rec2.t > t_max {
                    hit_rec2 = HitRecord::new(
                        hit_rec2.p,
                        t_max,
                        hit_rec2.u,
                        hit_rec2.v,
                        hit_rec2.mat_ptr,
                        hit_rec2.normal,
                        *r,
                    );
                }
                if hit_rec1.t >= hit_rec2.t {
                    return None;
                }
                if hit_rec1.t < 0.0 {
                    hit_rec1 = HitRecord::new(
                        hit_rec1.p,
                        0.0,
                        hit_rec1.u,
                        hit_rec1.v,
                        hit_rec1.mat_ptr,
                        hit_rec1.normal,
                        *r,
                    );
                }
                let ray_length = r.dir.length();
                let distance_inside_boundary = (hit_rec2.t - hit_rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * ((rbg as f32).ln()) as f64;
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = hit_rec1.t + hit_distance / ray_length;
                let p = r.at(t);
                let normal = Vec3::zero();
                Some(HitRecord {
                    p,
                    normal,
                    mat_ptr: &*self.phase_function,
                    t,
                    u: 0.0,
                    v: 0.0,
                    front_face: true,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct Iostropic {
    pub albedo: Arc<dyn Texture + Send + Sync>,
}

impl Iostropic {
    pub fn new_col(c: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }

    // pub fn new(a: Arc<dyn Texture + Send + Sync>) -> Self {
    //     Self { albedo: a }
    // }
}

impl Material for Iostropic {
    fn scatter(
        &self,
        r_in: &crate::Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<(crate::Ray, crate::vec3::Color)> {
        Some((
            Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.tm),
            self.albedo.value(rec.u, rec.v, &rec.p),
        ))
    }
}
