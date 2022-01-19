use std::ops::{Div, Mul, MulAssign};

use crate::vec3::Vec3;

use super::point::Point;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Direction(Vec3);

impl Direction {
  pub const fn new_from_vec3(input: Vec3) -> Self {
    Self(input)
  }
  pub const fn new(input: [f32; 3]) -> Self {
    Self::new_from_vec3(Vec3::new(input))
  }
  pub const fn x(&self) -> f32 {
    self.0.x()
  }
  pub const fn y(&self) -> f32 {
    self.0.y()
  }
  pub const fn z(&self) -> f32 {
    self.0.z()
  }
  pub fn unit_direction(&self) -> Self {
    Self::new_from_vec3(self.0.unit_vector())
  }
}

impl Mul<f32> for Direction {
  type Output = Self;

  fn mul(self, right_hand_side: f32) -> Self::Output {
    Self::new_from_vec3(right_hand_side * self.0)
  }
}

impl Mul<Direction> for f32 {
  type Output = Direction;

  fn mul(self, right_hand_side: Direction) -> Self::Output {
    right_hand_side * self
  }
}

impl Div<f32> for Direction {
  type Output = Self;

  fn div(self, right_hand_side: f32) -> Self::Output {
    Self::new_from_vec3(self.0 / right_hand_side)
  }
}

impl Div<Direction> for f32 {
  type Output = Direction;

  fn div(self, right_hand_side: Direction) -> Self::Output {
    Direction::new_from_vec3(self / Vec3::new([right_hand_side.x(), right_hand_side.y(), right_hand_side.z()]))
  }
}

impl MulAssign<f32> for Direction {
  fn mul_assign(&mut self, right_hand_side: f32) {
    self.0 = right_hand_side * self.0;
  }
}

impl From<Direction> for Point {
  fn from(starting_value: Direction) -> Self {
    Self::new([starting_value.x(), starting_value.y(), starting_value.z()])
  }
}
