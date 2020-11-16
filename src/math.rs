use rand::prelude::*;
use rand_distr::UnitSphere;

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

pub trait Random {
  fn random() -> Self;
  fn random_range(min: f64, max: f64) -> Self;
  fn random_unit() -> Self;
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
  fn random_unit() -> Vec3 {
    let v = UnitSphere.sample(&mut rand::thread_rng());
    Vec3::new(v[0], v[1], v[2])
  }
}

#[allow(dead_code)]
pub fn random_in_unit_sphere() -> Vec3 {
  loop {
    let p = Vec3::random_range(-1.0, 1.0);
    if p.mag_sq() >= 1.0 {
      continue;
    }
    return p;
  }
}

pub trait NearZero {
  fn near_zero(&self) -> bool;
}

impl NearZero for Vec3 {
  fn near_zero(&self) -> bool {
    let s = 1e-8;
    self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
  }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
  *v - 2.0 * (*v).dot(*n) * (*n)
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
  let cos_theta = (-uv.dot(*n)).min(1.0);
  let r_out_perp = etai_over_etat * (*uv + cos_theta * (*n));
  let r_out_parallel = (1.0 - r_out_perp.mag_sq()).abs().sqrt() * -1.0 * (*n);
  r_out_perp + r_out_parallel
}

#[test]
fn test_refract() {
  let eta_air = 1.0;
  let eta_glass = 1.5;
  {
    // Normal refract
    let uv = Vec3::new(1.0, -1.0, 0.0).normalized();
    let n = Vec3::new(0.0, 1.0, 0.0);
    let scattered = refract(&uv, &n, eta_air / eta_air);
    assert!((scattered - uv).mag_sq() < 1e-8);
  }
  {
    let uv = Vec3::new(1.0, -1.0, 0.0).normalized();
    let expected_scatter = Vec3 {
      x: 0.4714045207910316,
      y: -0.881917103688197,
      z: 0.0,
    };
    let n = Vec3::new(0.0, 1.0, 0.0);
    let scattered = refract(&uv, &n, eta_air / eta_glass);
    assert!((scattered - expected_scatter).mag_sq() < 1e-8);
  }
}
