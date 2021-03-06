use std::{
  fmt::Display,
  iter::Step,
  ops::{Add, Sub},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dimension(usize);

impl Dimension {
  pub const fn from_const(value: usize) -> Self {
    Dimension(value)
  }
}

impl Display for Dimension {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(formatter)
  }
}

impl From<usize> for Dimension {
  fn from(value: usize) -> Self {
    Dimension(value)
  }
}

impl From<Dimension> for usize {
  fn from(value: Dimension) -> Self {
    value.0
  }
}

impl From<Dimension> for f32 {
  #[allow(clippy::cast_precision_loss)]
  fn from(value: Dimension) -> Self {
    value.0 as f32
  }
}

impl From<f32> for Dimension {
  #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
  fn from(value: f32) -> Self {
    Self(value as usize)
  }
}

impl Step for Dimension {
  fn steps_between(start: &Self, end: &Self) -> Option<usize> {
    if end.0 >= start.0 {
      return Some(end.0 - start.0);
    }
    None
  }

  fn forward_checked(start: Self, count: usize) -> Option<Self> {
    start.0.checked_add(count).map(Dimension::from)
  }

  fn backward_checked(start: Self, count: usize) -> Option<Self> {
    start.0.checked_sub(count).map(Dimension::from)
  }
}

impl Add<Dimension> for Dimension {
  type Output = Self;

  fn add(self, right_hand_side: Dimension) -> Self::Output {
    Dimension::from(self.0 + right_hand_side.0)
  }
}

impl Sub<Dimension> for Dimension {
  type Output = Self;

  fn sub(self, right_hand_side: Dimension) -> Self::Output {
    Dimension::from(self.0 - right_hand_side.0)
  }
}
