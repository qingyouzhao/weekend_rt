extern crate image;

use std::rc::Rc;

use std::f32::INFINITY;

use image::imageops::flip_vertical_in_place;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod sphere;
use crate::camera::*;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::math::*;
use crate::ray::Ray;
use crate::sphere::Sphere;

fn ray_color(r: &Ray, world: Rc<dyn Hittable>) -> Color {
  let mut rec = HitRecord::default();
  if world.hit(&r, 0.0, INFINITY, &mut rec) {
    return Color(0.5 * (rec.normal + Vec3::broadcast(1.0)));
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

  // World
  let mut world = Rc::new(HittableList::default());
  Rc::make_mut(&mut world).add(Rc::new(Sphere::new(-Vec3::unit_z(), 0.5)));
  Rc::make_mut(&mut world).add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

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
      let pixel_color = ray_color(&r, world.clone());
      img.put_pixel(i, j, pixel_color.into());
    }
    bar.inc(1);
  }
  bar.finish();
  // We need to flip this to adhere to the same image we see in the tutorial
  flip_vertical_in_place(&mut img);
  img.save("test.png").unwrap();
}
