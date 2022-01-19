use std::{
  fmt::Display,
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::newtypes::distance::Distance;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3([f32; 3]);

impl Vec3 {
  pub const fn new(input: [f32; 3]) -> Self {
    Vec3(input)
  }

  pub const fn x(&self) -> f32 {
    self.0[0]
  }

  pub const fn y(&self) -> f32 {
    self.0[1]
  }

  pub const fn z(&self) -> f32 {
    self.0[2]
  }

  fn length_squared(&self) -> Distance {
    Distance::try_from_const(self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2])
      .expect("length squared must be positive")
  }

  pub fn length(&self) -> Distance {
    Distance::try_from_const(self.length_squared().as_f32().sqrt()).expect("length must be positive")
  }

  pub const fn dot(&self, right_hand_side: Self) -> f32 {
    self.x() * right_hand_side.x() + self.y() * right_hand_side.y() + self.z() * right_hand_side.z()
  }

  pub const fn cross(&self, right_hand_side: Self) -> Self {
    Self([
      self.y() * right_hand_side.z() - self.z() * right_hand_side.y(),
      self.z() * right_hand_side.x() - self.x() * right_hand_side.z(),
      self.x() * right_hand_side.y() - self.y() - right_hand_side.x(),
    ])
  }

  pub fn unit_vector(&self) -> Self {
    *self / self.length().as_f32()
  }
}

impl Display for Vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {} {}", self.x(), self.y(), self.z())
  }
}

impl Add for Vec3 {
  type Output = Self;

  fn add(self, right_hand_side: Self) -> Self::Output {
    Self([
      self.x() + right_hand_side.x(),
      self.y() + right_hand_side.y(),
      self.z() + right_hand_side.z(),
    ])
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, right_hand_side: Self) {
    self.0[0] = self.x() + right_hand_side.x();
    self.0[1] = self.y() + right_hand_side.y();
    self.0[2] = self.z() + right_hand_side.z();
  }
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, right_hand_side: Self) -> Self::Output {
    Self([
      self.x() - right_hand_side.x(),
      self.y() - right_hand_side.y(),
      self.z() - right_hand_side.z(),
    ])
  }
}

impl SubAssign for Vec3 {
  fn sub_assign(&mut self, right_hand_side: Self) {
    self.0[0] = self.x() - right_hand_side.x();
    self.0[1] = self.y() - right_hand_side.y();
    self.0[2] = self.z() - right_hand_side.z();
  }
}

impl Mul for Vec3 {
  type Output = Self;

  fn mul(self, right_hand_side: Self) -> Self::Output {
    Self([
      self.x() * right_hand_side.x(),
      self.y() * right_hand_side.y(),
      self.z() * right_hand_side.z(),
    ])
  }
}

impl MulAssign for Vec3 {
  fn mul_assign(&mut self, right_hand_side: Self) {
    self.0[0] = self.x() * right_hand_side.x();
    self.0[1] = self.y() * right_hand_side.y();
    self.0[2] = self.z() * right_hand_side.z();
  }
}

impl Mul<f32> for Vec3 {
  type Output = Self;

  fn mul(self, right_hand_side: f32) -> Self::Output {
    Self([
      self.x() * right_hand_side,
      self.y() * right_hand_side,
      self.z() * right_hand_side,
    ])
  }
}

impl Mul<Vec3> for f32 {
  type Output = Vec3;

  fn mul(self, right_hand_side: Vec3) -> Self::Output {
    right_hand_side * self
  }
}

impl MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, right_hand_side: f32) {
    self.0[0] *= right_hand_side;
    self.0[1] *= right_hand_side;
    self.0[2] *= right_hand_side;
  }
}

impl Div for Vec3 {
  type Output = Self;

  fn div(self, right_hand_side: Self) -> Self::Output {
    Vec3([
      self.x() / right_hand_side.x(),
      self.y() / right_hand_side.y(),
      self.z() / right_hand_side.z(),
    ])
  }
}

impl DivAssign for Vec3 {
  fn div_assign(&mut self, right_hand_side: Self) {
    self.0[0] = self.x() / right_hand_side.x();
    self.0[1] = self.y() / right_hand_side.y();
    self.0[2] = self.z() / right_hand_side.z();
  }
}

impl Div<f32> for Vec3 {
  type Output = Self;

  fn div(self, right_hand_side: f32) -> Self::Output {
    Self([
      self.x() / right_hand_side,
      self.y() / right_hand_side,
      self.z() / right_hand_side,
    ])
  }
}

impl Div<Vec3> for f32 {
  type Output = Vec3;

  fn div(self, right_hand_side: Vec3) -> Self::Output {
    Vec3([
      self / right_hand_side.x(),
      self / right_hand_side.y(),
      self / right_hand_side.z(),
    ])
  }
}

impl DivAssign<f32> for Vec3 {
  fn div_assign(&mut self, right_hand_side: f32) {
    self.0[0] /= right_hand_side;
    self.0[1] /= right_hand_side;
    self.0[2] /= right_hand_side;
  }
}

#[cfg(test)]
mod tests {
  use crate::newtypes::distance::Distance;

  use super::Vec3;

  #[test]
  fn add_vectors() {
    let mut vector_1 = Vec3::new([1.0, -1.0, 1.0]);
    let vector_2 = Vec3::new([-1.0, -1.0, 1.0]);
    assert_eq!(vector_1 + vector_2, Vec3::new([0.0, -2.0, 2.0]));
    vector_1 += vector_2;
    assert_eq!(vector_1, Vec3::new([0.0, -2.0, 2.0]));
  }
  #[test]
  fn subtract_vectors() {
    let mut vector_1 = Vec3::new([1.0, -1.0, 1.0]);
    let vector_2 = Vec3::new([-1.0, -1.0, 1.0]);
    assert_eq!(vector_1 - vector_2, Vec3::new([2.0, 0.0, 0.0]));
    vector_1 -= vector_2;
    assert_eq!(vector_1, Vec3::new([2.0, 0.0, 0.0]));
  }
  #[test]
  fn multiply_vector() {
    let mut vector_1 = Vec3::new([1.0, -2.0, 3.0]);
    let vector_2 = Vec3::new([-1.0, -2.0, 3.0]);
    assert_eq!(vector_1 * vector_2, Vec3::new([-1.0, 4.0, 9.0]));
    vector_1 *= vector_2;
    assert_eq!(vector_1, Vec3::new([-1.0, 4.0, 9.0]));
  }
  #[test]
  fn mulitply_f32() {
    let mut vector_1 = Vec3::new([1.0, -2.0, 3.0]);
    assert_eq!(vector_1 * -2.0, Vec3::new([-2.0, 4.0, -6.0]));
    assert_eq!(-2.0 * vector_1, Vec3::new([-2.0, 4.0, -6.0]));
    vector_1 *= -2.0;
    assert_eq!(vector_1, Vec3::new([-2.0, 4.0, -6.0]));
  }
  #[test]
  fn divide_vector() {
    let mut vector_1 = Vec3::new([1.0, -2.0, 3.0]);
    let vector_2 = Vec3::new([-0.5, -2.0, 1.0]);
    assert_eq!(vector_1 / vector_2, Vec3::new([-2.0, 1.0, 3.0]));
    vector_1 /= vector_2;
    assert_eq!(vector_1, Vec3::new([-2.0, 1.0, 3.0]));
  }
  #[test]
  fn divide_f32() {
    let mut vector_1 = Vec3::new([1.0, -2.0, 3.0]);
    assert_eq!(vector_1 / -2.0, Vec3::new([-0.5, 1.0, -1.5]));
    assert_eq!(-2.0 / vector_1, Vec3::new([-2.0, 1.0, -(2.0 / 3.0)]));
    vector_1 /= -2.0;
    assert_eq!(vector_1, Vec3::new([-0.5, 1.0, -1.5]));
  }
  #[test]
  fn length() {
    let vector = Vec3::new([3.0, 4.0, 0.0]);
    assert!(vector.length_squared().eq(&Distance::try_from_const(25.0).unwrap()));
    assert!(vector.length().eq(&Distance::try_from_const(5.0).unwrap()));
  }
  #[test]
  fn dot() {
    let vector_1 = Vec3::new([1.0, 2.0, 3.0]);
    let vector_2 = Vec3::new([3.0, 2.0, 1.0]);
    assert!(vector_1.dot(vector_2).eq(&10.0));
  }
  #[test]
  fn cross() {
    let vector_1 = Vec3::new([1.0, 2.0, 3.0]);
    let vector_2 = Vec3::new([4.0, 5.0, 6.0]);
    assert_eq!(vector_1.cross(vector_2), Vec3::new([-3.0, 6.0, -1.0]));
  }
  #[test]
  fn unit_vector() {
    let vector = Vec3::new([4.0, 0.0, 0.0]);
    assert_eq!(vector.unit_vector(), Vec3::new([1.0, 0.0, 0.0]));
  }
}
