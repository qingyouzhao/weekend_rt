use crate::math::*;
use crate::ray::Ray;

pub struct Camera {
  origin: Point,
  lower_left_corner: Point,
  horizontal: Vec3,
  vertical: Vec3,
}

impl Default for Camera {
  fn default() -> Self {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
      origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    Camera {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    }
  }
}

impl Camera {
  pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
    let theta = vfov.to_radians();
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (lookfrom - lookat).normalized();
    let u = vup.cross(w).normalized();
    let v = w.cross(u);

    let origin = lookfrom;
    let horizontal = viewport_width * u;
    let vertical = viewport_height * v;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
    Camera {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    Ray::new(
      &self.origin,
      &(self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin),
    )
  }
}
