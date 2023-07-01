// use std::env::temp_dir;
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::aarect::*;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::vec3::Point3;

pub struct Box_ {
    pub box_min: Point3,
    pub box_max: Point3,
    pub sides: HittableList,
}

impl Box_ {
    pub fn new(p0: Point3, p1: Point3, ptr: Arc<dyn Material + Send + Sync>) -> Self {
        let mut tmp_box: Box_ = Self {
            box_min: p0,
            box_max: p1,
            sides: HittableList::new(),
        };
        tmp_box.sides.add(Arc::new(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            ptr.clone(),
        )));

        tmp_box.sides.add(Arc::new(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            ptr.clone(),
        )));

        tmp_box.sides.add(Arc::new(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            ptr.clone(),
        )));

        tmp_box.sides.add(Arc::new(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            ptr.clone(),
        )));

        tmp_box.sides.add(Arc::new(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            ptr.clone(),
        )));

        tmp_box.sides.add(Arc::new(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            ptr.clone(),
        )));

        tmp_box
    }
}

impl Hittable for Box_ {
    fn bounding_box(&self, _: f64, _: f64) -> Option<crate::aabb::Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }

    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}
