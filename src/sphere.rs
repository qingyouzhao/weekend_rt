use std::option::Option;
use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::Point;
use crate::ray::Ray;

#[derive(Default, Clone)]
pub struct Sphere {
  center: Point,
  radius: f64,
  mat_rc: Option<Arc<dyn Material>>,
}

impl Sphere {
  pub fn new(cen: Point, r: f64, m: Arc<dyn Material>) -> Sphere {
    Sphere {
      center: cen,
      radius: r,
      mat_rc: Some(m),
    }
  }
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    // todo(qzhao): Can't we abstract this out with existing lib or math utils?
    let oc = r.origin() - self.center;
    let a = r.direction().mag_sq();
    let half_b = oc.dot(r.direction());
    let c = oc.mag_sq() - self.radius * self.radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
      return false;
    }
    let sqrtd = discriminant.sqrt();

    // Find the nearest root that lies in the acceptable range
    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root {
        return false;
      }
    }

    rec.t = root;
    rec.p = r.at(rec.t);
    let outward_normal = (rec.p - self.center) / self.radius;
    rec.set_face_normal(r, &outward_normal);
    rec.normal = (rec.p - self.center) / self.radius;
    rec.mat_rc = self.mat_rc.clone();

    true
  }
}
