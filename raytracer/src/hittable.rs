use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord<'m> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: &'m dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'m> HitRecord<'m> {
    pub fn new(p: Point3, t: f64, mat_ptr: &'m dyn Material, outward_normal: Vec3, r: Ray) -> Self {
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
            front_face,
        }
    }
}
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
