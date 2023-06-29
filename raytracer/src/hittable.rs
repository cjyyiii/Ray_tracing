use std::sync::Arc;

use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: &'a dyn Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        t: f64,
        u: f64,
        v: f64,
        mat_ptr: &'a dyn Material,
        outward_normal: Vec3,
        r: Ray,
    ) -> Self {
        let front_face: bool = Vec3::dot(r.dir, outward_normal) < 0.0;
        let normal: Vec3 = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            mat_ptr,
            t,
            u,
            v,
            front_face,
        }
    }

    // pub fn set_face_normal(&self, r: &Ray, outward_normal: Vec3) {
    //     let front_face: bool = Vec3::dot(r.dir, outward_normal) < 0.0;
    //     let normal = if front_face {
    //         outward_normal
    //     } else {
    //         -outward_normal
    //     };
    // }
}
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

pub struct Translate {
    pub ptr: Arc<dyn Hittable>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            ptr: p,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if let Some(mut output_box) = self.ptr.bounding_box(time0, time1) {
            output_box = Aabb::new(
                output_box.min() + self.offset,
                output_box.max() + self.offset,
            );
            Some(output_box)
        } else {
            None
        }
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r: Ray = Ray::new(r.orig - self.offset, r.dir, r.tm);
        if let Some(rec_hit) = self.ptr.hit(&moved_r, t_min, t_max) {
            let p = rec_hit.p + self.offset;
            Some(HitRecord::new(
                p,
                rec_hit.t,
                rec_hit.u,
                rec_hit.v,
                rec_hit.mat_ptr,
                rec_hit.normal,
                moved_r,
            ))
        } else {
            None
        }
    }
}

pub struct RotateY {
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub box_: Aabb,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle * std::f64::consts::PI / 180.0;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        if let Some(bbox) = p.bounding_box(0.0, 1.0) {
            let mut min = Point3::new(std::f64::INFINITY, std::f64::INFINITY, std::f64::INFINITY);
            let mut max = Point3::new(
                -std::f64::INFINITY,
                -std::f64::INFINITY,
                -std::f64::INFINITY,
            );
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * bbox.max().x() + (1.0 - i as f64) * bbox.min().x();
                        let y = j as f64 * bbox.max().y() + (1.0 - j as f64) * bbox.min().y();
                        let z = k as f64 * bbox.max().z() + (1.0 - k as f64) * bbox.min().z();

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);
                        let min_x = min.x().min(tester.x());
                        let max_x = max.x().max(tester.x());
                        let min_y = min.y().min(tester.y());
                        let max_y = max.y().max(tester.y());
                        let min_z = min.z().min(tester.z());
                        let max_z = max.z().max(tester.z());
                        min = Vec3::new(min_x, min_y, min_z);
                        max = Vec3::new(max_x, max_y, max_z);
                    }
                }
            }
            let box_ = Aabb::new(min, max);
            Self {
                ptr: p,
                sin_theta,
                cos_theta,
                hasbox: true,
                box_,
            }
        } else {
            Self {
                ptr: p,
                sin_theta,
                cos_theta,
                hasbox: false,
                box_: Aabb::new(Vec3::zero(), Vec3::zero()),
            }
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(self.box_.clone())
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.orig;
        let mut direction = r.dir;

        let origin_x = self.cos_theta * r.orig.x() - self.sin_theta * r.orig.z();
        let origin_z = self.sin_theta * r.orig.x() + self.cos_theta * r.orig.z();

        let direction_x = self.cos_theta * r.dir.x() - self.sin_theta * r.dir.z();
        let direction_z = self.sin_theta * r.dir.x() + self.cos_theta * r.dir.z();

        origin = Vec3::new(origin_x, origin.y(), origin_z);
        direction = Vec3::new(direction_x, direction.y(), direction_z);
        let rotated_r = Ray::new(origin, direction, r.tm);

        if let Some(rec_hit) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec_hit.p;
            let mut normal = rec_hit.normal;

            let p_x = self.cos_theta * rec_hit.p.x() + self.sin_theta * rec_hit.p.z();
            let p_z = -self.sin_theta * rec_hit.p.x() + self.cos_theta * rec_hit.p.z();

            let normal_x =
                self.cos_theta * rec_hit.normal.x() + self.sin_theta * rec_hit.normal.z();
            let normal_z =
                -self.sin_theta * rec_hit.normal.x() + self.cos_theta * rec_hit.normal.z();

            p = Vec3::new(p_x, p.y(), p_z);
            normal = Vec3::new(normal_x, normal.y(), normal_z);
            Some(HitRecord::new(
                p,
                rec_hit.t,
                rec_hit.u,
                rec_hit.v,
                rec_hit.mat_ptr,
                normal,
                rotated_r,
            ))
        } else {
            None
        }
    }
}
