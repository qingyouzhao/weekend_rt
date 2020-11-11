extern crate image;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

mod color;
mod math;
mod ray;
use crate::color::Color;
use crate::math::{Point, Vec3};
use crate::ray::Ray;

fn hit_sphere(center: &Point, radius: f32, r: &Ray) -> bool {
  let oc = r.origin() - *center;
  let a = r.direction().mag_sq();
  let b = 2.0 * oc.dot(r.direction());
  let c = oc.mag_sq() - radius * radius;
  let discriminant = b * b - 4.0 * a * c;
  discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color {
  let center = Vec3::new(0.0, 0.0, -1.0);
  let radius = 0.5;
  if hit_sphere(&center, radius, &r) {
    return Color(Vec3::new(1.0, 0.0, 0.0));
  }
  let unit_direction = r.direction().normalized();
  let t = 0.5 * (unit_direction.y + 1.0);
  Color((1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
  // Image
  let aspect_rasio = 16.0 / 9.0;
  let image_width: u32 = 400;
  let image_height: u32 = (image_width as f32 / aspect_rasio) as u32;

  // Camera
  let viewport_height = 2.0;
  let viewport_width = aspect_rasio * viewport_height;
  let focal_length = 1.0;

  let origin = Vec3::default();
  let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
  let vertical = Vec3::new(0.0, viewport_height, 0.0);
  let lower_left_corner =
    origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

  // Render
  let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

  let bar = ProgressBar::new(image_height as u64);
  for j in (0..image_height).rev() {
    for i in 0..image_width {
      let u = i as f32 / (image_width as f32 - 1.0);
      let v = j as f32 / (image_height as f32 - 1.0);
      let r = Ray::new(
        &origin,
        &(lower_left_corner + u * horizontal + v * vertical - origin),
      );
      let pixel_color = ray_color(&r);
      img.put_pixel(i, j, pixel_color.into());
    }
    bar.inc(1);
  }
  bar.finish();
  img.save("test.png").unwrap();
}
