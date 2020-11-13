use std::option::Option;
use std::sync::Arc;

use crate::material::Material;
use crate::math::*;
use crate::ray::Ray;

#[derive(Clone, Default)]
pub struct HitRecord {
  pub p: Point,
  pub normal: Vec3,
  pub mat_rc: Option<Arc<dyn Material>>,
  pub t: f64,
  pub front_face: bool,
}

impl HitRecord {
  pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
    self.front_face = r.direction().dot(*outward_normal) < 0.0;
    self.normal = if self.front_face {
      *outward_normal
    } else {
      -*outward_normal
    }
  }
}

pub trait Hittable: Send + Sync {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
