use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::vec3::Vec3;

use super::direction::Direction;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point(Vec3);

impl Point {
  pub const fn from_vec3_const(input: Vec3) -> Self {
    Self(input)
  }
  pub const fn from_f32_array_const(input: [f32; 3]) -> Self {
    Self::from_vec3_const(Vec3::new(input))
  }
  pub fn x(&self) -> f32 {
    self.0.x()
  }
  pub fn y(&self) -> f32 {
    self.0.y()
  }
  pub fn z(&self) -> f32 {
    self.0.z()
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, right_hand_side: Self) -> Self::Output {
    Self(self.0 + Vec3::new([right_hand_side.x(), right_hand_side.y(), right_hand_side.z()]))
  }
}

impl AddAssign for Point {
  fn add_assign(&mut self, right_hand_side: Self) {
    self.0 = self.0 + Vec3::new([right_hand_side.x(), right_hand_side.y(), right_hand_side.z()]);
  }
}

impl Sub for Point {
  type Output = Self;

  fn sub(self, right_hand_side: Self) -> Self::Output {
    Self(self.0 - Vec3::new([right_hand_side.x(), right_hand_side.y(), right_hand_side.z()]))
  }
}

impl SubAssign for Point {
  fn sub_assign(&mut self, right_hand_side: Self) {
    self.0 = self.0 - Vec3::new([right_hand_side.x(), right_hand_side.y(), right_hand_side.z()]);
  }
}

impl From<Point> for Direction {
  fn from(starting_value: Point) -> Self {
    Self::from_f32_array_const([starting_value.x(), starting_value.y(), starting_value.z()])
  }
}

impl From<Vec3> for Point {
  fn from(value: Vec3) -> Self {
    Self(value)
  }
}

impl From<[f32; 3]> for Point {
  fn from(value: [f32; 3]) -> Self {
    Self(Vec3::new(value))
  }
}

#[cfg(test)]
mod test {
  use super::Point;

  #[test]
  fn add_points() {
    let mut point_1 = Point::from_f32_array_const([1.0, -1.0, 1.0]);
    let point_2 = Point::from_f32_array_const([-1.0, -1.0, 1.0]);
    assert_eq!(point_1 + point_2, Point::from_f32_array_const([0.0, -2.0, 2.0]));
    point_1 += point_2;
    assert_eq!(point_1, Point::from_f32_array_const([0.0, -2.0, 2.0]));
  }
  #[test]
  fn subtract_points() {
    let mut point_1 = Point::from_f32_array_const([1.0, -1.0, 1.0]);
    let point_2 = Point::from_f32_array_const([-1.0, -1.0, 1.0]);
    assert_eq!(point_1 - point_2, Point::from_f32_array_const([2.0, 0.0, 0.0]));
    point_1 -= point_2;
    assert_eq!(point_1, Point::from_f32_array_const([2.0, 0.0, 0.0]));
  }
}
