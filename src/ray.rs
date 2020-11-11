use crate::math::Point3;
use crate::math::Vec3;

#[derive(Default)]
pub struct Ray {
  orig: Point3,
  dir: Vec3,
}

impl Ray {
  // todo(zqy): I don't know how to do a default constructor yet ...
  pub fn new(origin: &Point3, direction: &Vec3) -> Ray {
    Ray {
      orig: *origin,
      dir: *direction,
    }
  }

  pub fn origin(&self) -> Point3 {
    self.orig
  }

  pub fn direction(&self) -> Vec3 {
    self.dir
  }

  // todo Vec3 only support f32 not f64, let's figure out if this is a problem later
  pub fn at(&self, t: f32) -> Point3 {
    self.orig + self.dir * t
  }
}
