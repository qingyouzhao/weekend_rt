use crate::math::Vec3;

#[derive(Default)]
pub struct Color(pub Vec3);

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
