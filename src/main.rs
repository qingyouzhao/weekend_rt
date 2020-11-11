use std::f64::INFINITY;
use std::sync::{Arc, Mutex};

extern crate image;
use image::imageops::flip_vertical_in_place;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::prelude::*;
use rayon::prelude::*;

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

fn ray_color(r: &Ray, world: Arc<dyn Hittable>, depth: u32) -> Vec3 {
  if depth == 0 {
    return Vec3::zero();
  }
  let mut rec = HitRecord::default();
  if world.hit(&r, 0.001, INFINITY, &mut rec) {
    let target = rec.p + rec.normal + Vec3::random_unit();
    return 0.5 * ray_color(&Ray::new(&rec.p, &(target - rec.p)), world, depth - 1);
  }
  let unit_direction = r.direction().normalized();
  let t = 0.5 * (unit_direction.y + 1.0);
  (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
  // Image
  let aspect_rasio = 16.0 / 9.0;
  let image_width: u32 = 400;
  let image_height: u32 = (image_width as f64 / aspect_rasio) as u32;
  let samples_per_pixel = 100;
  let max_depth = 50;

  // World
  let mut world = Arc::new(HittableList::default());
  Arc::make_mut(&mut world).add(Arc::new(Sphere::new(-Vec3::unit_z(), 0.5)));
  Arc::make_mut(&mut world).add(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

  // Camera
  let cam = Camera::default();

  // Renderimg
  let arc_img = Arc::new(Mutex::new(ImageBuffer::<Rgb<u8>, Vec<_>>::new(
    image_width,
    image_height,
  )));

  let bar = ProgressBar::new(image_height as u64);
  for j in (0..image_height).rev() {
    (0..image_width).into_par_iter().for_each(|i| {
      let mut pixel_color = MultiSampleColor {
        color: Vec3::zero(),
        samples_per_pixel: samples_per_pixel,
      };

      for _ in 0..pixel_color.samples_per_pixel {
        let mut rng = rand::thread_rng();
        let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
        let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
        let r = cam.get_ray(u, v);
        pixel_color.color += ray_color(&r, world.clone(), max_depth);
      }
      let mut img = arc_img.lock().unwrap();
      img.put_pixel(i, j, pixel_color.into());
    });
    bar.inc(1);
  }
  bar.finish();
  let img_mutex = Arc::try_unwrap(arc_img)
    .expect("We should have finished processing the image, why is it still locked?");
  let mut final_img = img_mutex
    .into_inner()
    .expect("Why is my image still locked");
  flip_vertical_in_place(&mut final_img);
  // flip_vertical_in_place(final_img);
  final_img.save("test.png").unwrap();
}
