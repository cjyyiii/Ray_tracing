use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::Ray;
use rand::Rng;
// use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    box_: Aabb,
}

impl BVHNode {
    pub fn new_boxed(list: HittableList, time0: f64, time1: f64) -> Arc<dyn Hittable> {
        BVHNode::init(list.hittable_list, time0, time1)
    }

    pub fn init(
        mut src_objects: Vec<Arc<dyn Hittable>>,
        time0: f64,
        time1: f64,
    ) -> Arc<dyn Hittable> {
        // fn box_compare(
        //     time0: f64,
        //     time1: f64,
        //     axis: usize,
        // ) -> impl FnMut(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering {
        //     move |a, b| {
        //         let box_a: Option<Aabb> = a.bounding_box(time0, time1);
        //         let box_b: Option<Aabb> = b.bounding_box(time0, time1);
        //         if let (Some(a), Some(b)) = (box_a, box_b) {
        //             let ac = a.minimum[axis as i32] + a.maximum[axis as i32];
        //             let bc = b.minimum[axis as i32] + b.maximum[axis as i32];
        //             ac.partial_cmp(&bc).unwrap()
        //         } else {
        //             panic!["No bounding box in bvh_node constructor"]
        //         }
        //     }
        // }

        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let axis: i32 = rng.gen_range(0..3);

        let object_span: usize = src_objects.len();
        match object_span {
            0 => panic!["No bounding box in bvh_node constructor."],
            1 => src_objects.remove(0),
            _ => {
                src_objects.sort_by(|a, b| {
                    a.bounding_box(time0, time1).unwrap().minimum[axis]
                        .partial_cmp(&b.bounding_box(time0, time1).unwrap().minimum[axis])
                        .unwrap()
                });
                let mut left_objects: Vec<Arc<dyn Hittable>> = src_objects;
                let right_objects: Vec<Arc<dyn Hittable>> =
                    left_objects.split_off(left_objects.len() / 2);
                let left: Arc<dyn Hittable> = Self::init(left_objects, time0, time1);
                let right: Arc<dyn Hittable> = Self::init(right_objects, time0, time1);
                let box_: Aabb = Aabb::surrounding_box(
                    &left.bounding_box(time0, time1).unwrap(),
                    &right.bounding_box(time0, time1).unwrap(),
                );
                Arc::new(Self { left, right, box_ })
            }
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.box_.hit(r, t_min, t_max) {
            return None;
        }

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
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let output_box: Aabb = self.box_.clone();
        Some(output_box)
    }
}
