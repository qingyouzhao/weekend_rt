extern crate image;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use indicatif::ProgressBar;

mod math;
use crate::math::Vec3;

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

      let ir: u8 = (255.99 * r) as u8;
      let ig: u8 = (255.99 * g) as u8;
      let ib: u8 = (255.99 * b) as u8;
      img.put_pixel(i, j, image::Rgb::<u8>([ir, ig, ib]));
    }
    bar.inc(1);
  }
  bar.finish();
  img.save("test.png").unwrap();
}
