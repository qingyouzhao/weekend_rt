use crate::color::*;
use crate::hittable::HitRecord;
use crate::math::*;
use crate::ray::Ray;

pub trait Material: Send + Sync {
  fn scatter(
    &self,
    r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
  ) -> bool;
}

pub struct Lambertian {
  albedo: Color,
}

impl Lambertian {
  pub fn new(a: &Color) -> Lambertian {
    Lambertian { albedo: *a }
  }
}

// Something is wrong with this lanbertian reflection
impl Material for Lambertian {
  fn scatter(
    &self,
    _r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
  ) -> bool {
    let mut scatter_direction = rec.normal + Vec3::random_unit();
    if scatter_direction.near_zero() {
      scatter_direction = rec.normal;
    }
    *scattered = Ray::new(&rec.p, &scatter_direction);
    *attenuation = self.albedo.0;
    true
  }
}

pub struct Metal {
  albedo: Color,
}

impl Metal {
  pub fn new(a: &Color) -> Metal {
    Metal { albedo: *a }
  }
}

impl Material for Metal {
  fn scatter(
    &self,
    r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
  ) -> bool {
    let reflected = reflect(&r_in.direction().normalized(), &rec.normal);
    *scattered = Ray::new(&rec.p, &reflected);
    *attenuation = self.albedo.0;
    scattered.direction().dot(rec.normal) > 0.0
  }
}
