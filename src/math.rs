use rand::distributions::Uniform;
use rand::prelude::*;
use rand::RngCore;

pub type Vec3 = ultraviolet::DVec3;
pub type Point = Vec3;

pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
  if x < min {
    return min;
  }
  if x > max {
    return max;
  }
  x
}

trait Random {
  fn random() -> Self;
  fn random_range(min: f64, max: f64) -> Self;
}

impl Random for Vec3 {
  fn random() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen(), rng.gen(), rng.gen())
  }
  fn random_range(min: f64, max: f64) -> Self {
    let mut rng = rand::thread_rng();
    Vec3::new(
      rng.gen_range(min, max),
      rng.gen_range(min, max),
      rng.gen_range(min, max),
    )
  }
}

pub fn random_in_unit_sphere() -> Vec3 {
  loop {
    let p = Vec3::random_range(-1.0, 1.0);
    if p.mag_sq() < 1.0 {
      return p;
    }
  }
}
