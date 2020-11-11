use crate::hittable::{HitResult, Hittable};
use crete::math::Point;

#[derive(Default)]
struct Sphere {
  center: Point,
  radius: f32,
}

impl Sphere {
  fn new(cen: Point, r: f32) -> Sphere {
    Sphere {
      center: cen,
      radius: r,
    }
  }
}

impl Hittable for Sphere{
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: mut& HitRecord) -> bool{
    let oc = r.origin() - self.center;
    let a = r.direction().mag_sq();
    let half_b = oc.dot(r.direction());
    let c = oc.mag_sq() - radius * radius;

    let discriminant = half_b*half_b - a *c;
    if discriminant< 0.0 {
      return false;
    } 
    let sqrtd = d.sqrt();

    // Find the nearest root that lies in the acceptable range
    let root = (-half_b - sqrtd) / a;
    if root<t_min || t_max > root{
      root = (-half_b + sqrtd) / a;
      if (root < t_min || t_max < root){
        return false;
      }
    }

    rec.t = root;
    rec.p = r.at(rec.t);
    rec.normal = (rec.p - center) / radius;

    return true; 
  }
}
