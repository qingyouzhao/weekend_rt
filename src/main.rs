extern crate image;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

mod color;
mod math;
mod ray;
use crate::color::Color;
use crate::math::Vec3;
use crate::ray::Ray;

fn ray_color(r: &Ray) -> Color {
  let unit_direction = r.direction().normalized();
  let t = 0.5 * (unit_direction.y + 1.0);
  Color((1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
  println!("Hello, world!");
  let image_width: u32 = 256;
  let image_height: u32 = 256;

  let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

  let bar = ProgressBar::new(image_height as u64);
  for j in 0..image_height {
    for i in 0..image_width {
      let r = (i as f32) / (image_width - 1) as f32;
      let g = (j as f32) / (image_height - 1) as f32;
      let b = 0.25;
      let c = Color(Vec3::new(r, g, b));
      img.put_pixel(i, j, c.into());
    }
    bar.inc(1);
  }
  bar.finish();
  img.save("test.png").unwrap();
}
