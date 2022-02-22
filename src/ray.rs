use crate::{
  newtypes::{
    color::{Color, Rgb},
    direction::Direction,
    percent::Strict as StrictPercent,
    point::Point,
  },
  sphere::Sphere,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ray {
  origin: Point,
  direction: Direction,
}

impl Ray {
  pub const fn new(origin: Point, direction: Direction) -> Self {
    Ray { origin, direction }
  }
  pub const fn origin(&self) -> Point {
    self.origin
  }
  pub const fn direction(&self) -> Direction {
    self.direction
  }
  pub fn _at(&self, distance: f32) -> Point {
    self.origin + (distance * self.direction).into()
  }
  pub fn find_color(&self, sphere: &Sphere) -> Rgb {
    if sphere.does_ray_hit_sphere(*self) {
      return Rgb::new([255.0, 0.0, 0.0]).expect("A red color");
    }
    let unit_direction = self.direction().unit_direction();
    let color_lerp = StrictPercent::new(0.5 * (unit_direction.y() + 1.0)).expect("Must be a valid percent");
    Rgb::new_from_percent([
      StrictPercent::new(0.5).unwrap(),
      StrictPercent::new(0.7).unwrap(),
      StrictPercent::new(1.0).unwrap(),
    ])
    .expect("this is a valid blue color")
    .blend_two_colors(&Rgb::white(), color_lerp)
  }
}
