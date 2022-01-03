use std::fmt::Display;

use crate::{error::LocalError, vec3::Vec3};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color(Vec3);

const fn is_color_dimension_valid(value_to_check: f32) -> bool {
  if value_to_check >= 0.0 && value_to_check <= 256.0 {
    return true;
  }
  false
}
impl Color {
  pub fn new_from_vec3(input: Vec3) -> Result<Self, LocalError> {
    if !is_color_dimension_valid(input.x()) || !is_color_dimension_valid(input.y()) || !is_color_dimension_valid(input.x()) {
      return Err("Color dimensions must be between 0 and 255".into());
    }
    Ok(Self(input))
  }
  pub fn new(input: [f32; 3]) -> Result<Self, LocalError> {
    Self::new_from_vec3(Vec3::new(input))
  }
  #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
  pub fn red(&self) -> u16 {
    self.0.x() as u16
  }
  #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
  pub fn blue(&self) -> u16 {
    self.0.y() as u16
  }
  #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
  pub fn green(&self) -> u16 {
    self.0.z() as u16
  }
}

impl Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {} {}", self.red(), self.blue(), self.green())
  }
}
