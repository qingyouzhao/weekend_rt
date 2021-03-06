use crate::math::*;

#[derive(Default, Copy, Clone)]
pub struct Color(pub Vec3);

#[derive(Default)]
pub struct MultiSampleColor {
  pub color: Vec3,
  pub samples_per_pixel: u32,
}

// Should I use this trait or should I use Color as itself?
impl From<Color> for image::Rgb<u8> {
  fn from(c: Color) -> Self {
    image::Rgb::<u8>([
      (255.999 * c.0.x) as u8,
      (255.999 * c.0.y) as u8,
      (255.999 * c.0.z) as u8,
    ])
  }
}

impl From<MultiSampleColor> for image::Rgb<u8> {
  fn from(c: MultiSampleColor) -> Self {
    let scale = 1.0 / c.samples_per_pixel as f64;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    image::Rgb::<u8>([
      (255.999 * clamp((c.color.x * scale).sqrt(), 0.0, 0.999)) as u8,
      (255.999 * clamp((c.color.y * scale).sqrt(), 0.0, 0.999)) as u8,
      (255.999 * clamp((c.color.z * scale).sqrt(), 0.0, 0.999)) as u8,
    ])
  }
}
