use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
// use crate::material::Material;
use crate::ray::Ray;
// use crate::sphere::MovingSphere;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HittableList {
    pub hittable_list: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            hittable_list: Vec::new(),
        }
    }

    // pub fn clear(&mut self) {
    //     self.hittable_list.clear();
    // }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.hittable_list.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far: f64 = t_max;

        for object in &self.hittable_list {
            if let Some(hit_rec) = object.hit(r, t_min, closest_so_far) {
                hit_anything = Some(hit_rec.clone());
                closest_so_far = hit_rec.t;
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.hittable_list.is_empty() {
            return None;
        }

        let mut output_box: Aabb = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut first_box: bool = true;

        for object in &self.hittable_list {
            if let Some(temp_box) = object.bounding_box(time0, time1) {
                output_box = if first_box {
                    temp_box
                } else {
                    Aabb::surrounding_box(output_box, temp_box)
                };
                first_box = false;
            } else {
                return None;
            }
        }
        Some(output_box)
    }
}
