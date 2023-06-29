use crate::{
    aabb::Aabb,
    hittable::*,
    material::Material,
    vec3::{Point3, Vec3},
    Ray,
};
use std::sync::Arc;

pub struct XyRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp: mat,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.z()) / r.dir.z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x() + t * r.dir.x();
        let y = r.orig.y() + t * r.dir.y();
        if self.x0 > x || self.x1 < x || self.y0 > y || self.y1 < y {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        Some(HitRecord::new(
            r.at(t),
            t,
            u,
            v,
            &*self.mp,
            outward_normal,
            *r,
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        Some(output_box)
    }
}

pub struct YzRect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mp: mat,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.x()) / r.dir.x();
        if t < t_min || t > t_max {
            return None;
        }
        let z = r.orig.z() + t * r.dir.z();
        let y = r.orig.y() + t * r.dir.y();
        if self.z0 > z || self.z1 < z || self.y0 > y || self.y1 < y {
            return None;
        }
        let u = (z - self.z0) / (self.z1 - self.z0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        Some(HitRecord::new(
            r.at(t),
            t,
            u,
            v,
            &*self.mp,
            outward_normal,
            *r,
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        Some(output_box)
    }
}

pub struct XzRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mp: mat,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.y()) / r.dir.y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x() + t * r.dir.x();
        let z = r.orig.z() + t * r.dir.z();
        if self.x0 > x || self.x1 < x || self.z0 > z || self.z1 < z {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        Some(HitRecord::new(
            r.at(t),
            t,
            u,
            v,
            &*self.mp,
            outward_normal,
            *r,
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        let output_box = Aabb::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        Some(output_box)
    }
}
