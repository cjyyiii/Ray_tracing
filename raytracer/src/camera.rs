// mod rtweekend;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w: Vec3 = Vec3::unit_vector(lookfrom - lookat);
        let u: Vec3 = Vec3::unit_vector(Vec3::cross(vup, w));
        let v: Vec3 = Vec3::cross(w, u);

        let origin: Vec3 = lookfrom;
        let horizontal: Vec3 = focus_dist * viewport_width * u;
        let vertical: Vec3 = focus_dist * viewport_height * v;
        let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius: f64 = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = self.lens_radius * Vec3::random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
