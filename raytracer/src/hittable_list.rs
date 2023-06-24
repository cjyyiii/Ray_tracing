use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

pub struct HittableList {
    pub hittable_list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            hittable_list: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.hittable_list.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far: f64 = t_max;

        for object in &self.hittable_list {
            if let Some(hit_rec) = object.hit(r, t_min, closest_so_far) {
                hit_anything = Some(hit_rec);
                closest_so_far = hit_rec.t;
            }
        }

        hit_anything
    }
}
