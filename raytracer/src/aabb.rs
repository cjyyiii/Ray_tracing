use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl Aabb {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Aabb {
            minimum: a,
            maximum: b,
        }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let invd = 1.0 / r.dir[a];
            let mut t0 = (self.minimum[a] - r.orig[a]) * invd;
            let mut t1 = (self.maximum[a] - r.orig[a]) * invd;
            if invd < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = if t_min > t0 { t_min } else { t0 };
            t_max = if t_max > t1 { t1 } else { t_min };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Self {
        let small: Point3 = Point3::new(
            box0.minimum.x().min(box1.minimum.x()),
            box0.minimum.y().min(box1.minimum.y()),
            box0.minimum.z().min(box1.minimum.z()),
        );
        let big: Point3 = Point3::new(
            box0.maximum.x().max(box1.maximum.x()),
            box0.maximum.y().max(box1.maximum.y()),
            box0.maximum.z().max(box1.maximum.z()),
        );
        Aabb::new(small, big)
    }
}
