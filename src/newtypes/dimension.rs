use std::{
  fmt::Display,
  iter::Step,
  ops::{Add, Deref, DerefMut, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dimension(usize);

impl Dimension {
  pub const fn from_const(value: usize) -> Self {
    Dimension(value)
  }
}

impl Deref for Dimension {
  type Target = usize;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Dimension {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
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

impl Step for Dimension {
  fn steps_between(start: &Self, end: &Self) -> Option<usize> {
    if end.0 >= start.0 {
      return Some(**end - **start);
    }
    None
  }

  fn forward_checked(start: Self, count: usize) -> Option<Self> {
    start.checked_add(count).map(Dimension::from)
  }

  fn backward_checked(start: Self, count: usize) -> Option<Self> {
    start.checked_sub(count).map(Dimension::from)
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
