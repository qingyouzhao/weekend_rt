use crate::ray::Ray;
use crate::math::Point;

#[derive(Default)]
pub struct HitRecord{
  Point p,
  Vec3 normal,
  f32 t
}

trait Hittable{
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: mut& HitRecord) -> bool;
}