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
  fuzz: f64,
}

impl Metal {
  pub fn new(a: &Color, f: f64) -> Metal {
    Metal {
      albedo: *a,
      fuzz: f.min(1.0),
    }
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
    *scattered = Ray::new(&rec.p, &(reflected + self.fuzz * Vec3::random_unit()));
    *attenuation = self.albedo.0;
    scattered.direction().dot(rec.normal) > 0.0
  }
}

pub struct Dielectric {
  ir: f64, // index of refraction
}

impl Dielectric {
  pub fn new(index_of_refraction: f64) -> Self {
    Dielectric {
      ir: index_of_refraction,
    }
  }
}

impl Material for Dielectric {
  fn scatter(
    &self,
    r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
  ) -> bool {
    *attenuation = Vec3::one();
    let refraction_ratio = if rec.front_face {
      1.0 / self.ir
    } else {
      self.ir
    };

    let unit_direction = r_in.direction().normalized();
    let cos_theta = (-unit_direction.dot(rec.normal)).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    let direction = if cannot_refract {
      reflect(&unit_direction, &rec.normal)
    } else {
      refract(&unit_direction, &rec.normal, refraction_ratio)
    };
    *scattered = Ray::new(&rec.p, &direction);
    true
  }
}
