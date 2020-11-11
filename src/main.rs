use std::f32::INFINITY;
use std::rc::Rc;

extern crate image;
use image::imageops::flip_vertical_in_place;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::prelude::*;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod sphere;
use crate::camera::*;
use crate::color::*;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::math::*;
use crate::ray::Ray;
use crate::sphere::Sphere;

fn ray_color(r: &Ray, world: Rc<dyn Hittable>) -> Vec3 {
  let mut rec = HitRecord::default();
  if world.hit(&r, 0.0, INFINITY, &mut rec) {
    return 0.5 * (rec.normal + Vec3::broadcast(1.0));
  }
  let unit_direction = r.direction().normalized();
  let t = 0.5 * (unit_direction.y + 1.0);
  (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
  let mut rng = rand::thread_rng();

  // Image
  let aspect_rasio = 16.0 / 9.0;
  let image_width: u32 = 400;
  let image_height: u32 = (image_width as f32 / aspect_rasio) as u32;
  let samples_per_pixel = 100;

  // World
  let mut world = Rc::new(HittableList::default());
  Rc::make_mut(&mut world).add(Rc::new(Sphere::new(-Vec3::unit_z(), 0.5)));
  Rc::make_mut(&mut world).add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

  // Camera
  let cam = Camera::default();

  // Render
  let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

  let bar = ProgressBar::new(image_height as u64);
  for j in (0..image_height).rev() {
    for i in 0..image_width {
      let mut pixel_color = MultiSampleColor {
        color: Vec3::zero(),
        samples_per_pixel: samples_per_pixel,
      };

      for _ in 0..pixel_color.samples_per_pixel {
        let u = (i as f32 + rng.gen::<f32>()) / (image_width as f32 - 1.0);
        let v = (j as f32 + rng.gen::<f32>()) / (image_height as f32 - 1.0);
        let r = cam.get_ray(u, v);
        pixel_color.color += ray_color(&r, world.clone());
      }
      img.put_pixel(i, j, pixel_color.into());
    }
    bar.inc(1);
  }
  bar.finish();
  // We need to flip this to adhere to the same image we see in the tutorial
  flip_vertical_in_place(&mut img);
  img.save("test.png").unwrap();
}
