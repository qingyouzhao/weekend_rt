use crate::hittable::HitRecord;
use crate::math::*;
use crate::ray::Ray;

pub trait Material {
  fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}
