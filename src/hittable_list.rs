use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct HittableList {
  objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
  pub fn new(&self, object: Rc<dyn Hittable>) -> HittableList {
    HittableList {
      objects: vec![object],
    }
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }

  pub fn add(&mut self, object: Rc<dyn Hittable>) {
    self.objects.push(object);
  }
}

impl Hittable for HittableList {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool
  where
    Self: Sized,
  {
    let mut temp_rec = HitRecord::default();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for object in self.objects.iter() {
      if object.hit(&r, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        *rec = temp_rec;
      }
    }
    hit_anything
  }
}
