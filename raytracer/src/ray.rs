use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    // pub fn origin(&self) -> Vec3 {
    //     self.orig
    // }

    // pub fn direction(&self) -> Vec3 {
    //     self.dir
    // }

    pub fn new(orig: Vec3, dir: Vec3, tm: f64) -> Self {
        Self { orig, dir, tm }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}
