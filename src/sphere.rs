use crate::{
  newtypes::{direction::Direction, distance::Distance, point::Point},
  ray::Ray,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Sphere {
  center: Point,
  radius: Distance,
}

impl Sphere {
  pub const fn new_const(center: Point, radius: Distance) -> Self {
    Self { center, radius }
  }
  pub const fn center(&self) -> Point {
    self.center
  }
  pub const fn radius(&self) -> Distance {
    self.radius
  }
  pub fn does_ray_hit_sphere(&self, ray: Ray) -> bool {
    let center_ray_offset: Direction = (ray.origin() - self.center()).into();
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * center_ray_offset.dot(ray.direction());
    let c = center_ray_offset.dot(center_ray_offset) - self.radius().as_f32() * self.radius().as_f32();
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
  }
}
