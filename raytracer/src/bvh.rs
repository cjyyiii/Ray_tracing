use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use rand::Rng;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    box_: Aabb,
}

impl BVHNode {
    pub fn new_boxed(
        list: HittableList,
        time0: f64,
        time1: f64,
    ) -> Arc<dyn Hittable + Send + Sync> {
        BVHNode::build(list.hittable_list, time0, time1)
    }

    pub fn build(
        src_objects: Vec<Arc<dyn Hittable + Send + Sync>>,
        time0: f64,
        time1: f64,
    ) -> Arc<dyn Hittable + Send + Sync> {
        let axis: i32 = rand::thread_rng().gen_range(0..3);
        let mut objects = src_objects;

        objects.sort_by(|a, b| {
            a.bounding_box(time0, time1).unwrap().min()[axis]
                .partial_cmp(&b.bounding_box(time0, time1).unwrap().min()[axis])
                .unwrap()
        }); //基于https://github.com/JolyneFr/RayTracing/blob/master/src/bvh.rs

        let mut left = objects[0].clone();
        let mut right = objects[0].clone();
        let object_span: usize = objects.len();
        match object_span {
            1 => {}
            2 => right = objects[1].clone(),
            _ => {
                let mut l: Vec<Arc<dyn Hittable + Send + Sync>> = objects;
                let r: Vec<Arc<dyn Hittable + Send + Sync>> = l.split_off(l.len() / 2);
                left = BVHNode::build(l, time0, time1);
                right = BVHNode::build(r, time0, time1);
            }
        };
        let box_: Aabb = Aabb::surrounding_box(
            left.bounding_box(time0, time1).unwrap(),
            right.bounding_box(time0, time1).unwrap(),
        );
        Arc::new(Self { left, right, box_ })
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.box_.hit(r, t_min, t_max) {
            let hit_left: Option<HitRecord<'_>> = self.left.hit(r, t_min, t_max);
            let hit_right: Option<HitRecord<'_>> = self.right.hit(r, t_min, t_max);
            match (hit_left, hit_right) {
                (Some(hit_left), Some(hit_right)) => {
                    if hit_left.t < hit_right.t {
                        Some(hit_left)
                    } else {
                        Some(hit_right)
                    }
                }
                (Some(hit_left), None) => Some(hit_left),
                (None, Some(hit_right)) => Some(hit_right),
                (None, None) => None,
            }
        } else {
            None
        }
    } //基于https://github.com/JolyneFr/RayTracing/blob/master/src/bvh.rs

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let output_box: Aabb = self.box_.clone();
        Some(output_box)
    }
}
