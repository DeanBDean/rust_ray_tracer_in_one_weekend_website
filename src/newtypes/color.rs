use std::{error::Error, fmt::Display};

use crate::vec3::Vec3;

use super::percent::Strict as StrictPercent;

pub const MINIMUM_VALID_RGB_COLOR: f32 = 0.0;
pub const MAXIUM_VALID_RGB_COLOR: f32 = 256.0;

pub trait Color: Sized {
  fn adjust_color_by_percent(&self, percent: StrictPercent) -> Self;
  fn blend_two_colors(&self, color_to_blend: &Self, percent_of_initial_color: StrictPercent) -> Self;
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct StateAtError {
  message: String,
  red: f32,
  green: f32,
  blue: f32,
}

impl StateAtError {
  pub const fn new(colors: Vec3, message: String) -> Self {
    Self {
      message,
      red: colors.x(),
      green: colors.y(),
      blue: colors.z(),
    }
  }
  pub fn message(&self) -> String {
    self.message.clone()
  }
  pub const fn red(&self) -> f32 {
    self.red
  }
  pub const fn green(&self) -> f32 {
    self.green
  }
  pub const fn blue(&self) -> f32 {
    self.blue
  }
}

impl Display for StateAtError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}: red:{},green:{},blue:{}",
      self.message(),
      self.red(),
      self.green(),
      self.blue()
    )
  }
}

impl Error for StateAtError {}

#[allow(unsafe_code)]
unsafe impl Send for StateAtError {}
#[allow(unsafe_code)]
unsafe impl Sync for StateAtError {}

#[derive(Debug)]
pub enum Errors {
  Below(Box<dyn Error + Send + Sync + 'static>),
  Above(Box<dyn Error + Send + Sync + 'static>),
  AboveAndBelow(Box<dyn Error + Send + Sync + 'static>),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rgb(Vec3);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ColorValidationStates {
  Above,
  Below,
  Valid,
}

const fn is_color_dimension_valid(value_to_check: f32) -> ColorValidationStates {
  if value_to_check < MINIMUM_VALID_RGB_COLOR {
    return ColorValidationStates::Below;
  }

  if value_to_check > MAXIUM_VALID_RGB_COLOR {
    return ColorValidationStates::Above;
  }

  ColorValidationStates::Valid
}

const fn convert_strict_percent_to_color_value(percent: StrictPercent) -> f32 {
  255.999 * percent.as_f32()
}
impl Rgb {
  pub fn new_from_vec3(input: Vec3) -> Result<Self, Errors> {
    let validation_states = [
      is_color_dimension_valid(input.x()),
      is_color_dimension_valid(input.y()),
      is_color_dimension_valid(input.z()),
    ];
    let any_values_above = validation_states.iter().any(|value| *value == ColorValidationStates::Above);
    let any_values_below = validation_states.iter().any(|value| *value == ColorValidationStates::Below);
    if any_values_above && any_values_below {
      return Err(Errors::AboveAndBelow(Box::new(StateAtError::new(
        input,
        "Some color values are above and some are below the valid limit".to_string(),
      ))));
    }

    if any_values_above {
      return Err(Errors::Above(Box::new(StateAtError::new(
        input,
        "Some color value or values are above the highest limit".to_string(),
      ))));
    }
    if any_values_below {
      return Err(Errors::Below(Box::new(StateAtError::new(
        input,
        "Some color value or values are below the highest limit".to_string(),
      ))));
    }
    Ok(Self(input))
  }
  pub fn new(input: [f32; 3]) -> Result<Self, Errors> {
    Self::new_from_vec3(Vec3::new(input))
  }
  pub fn new_from_percent(input: [StrictPercent; 3]) -> Result<Self, Errors> {
    Self::new(input.map(convert_strict_percent_to_color_value))
  }
  #[allow(dead_code)]
  pub fn black() -> Self {
    Self::new([0.0, 0.0, 0.0]).expect("This is a valid black color")
  }
  pub fn white() -> Self {
    Self::new([255.0, 255.0, 255.0]).expect("This is a valid white color")
  }
  #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
  pub const fn red_part(&self) -> u16 {
    self.0.x() as u16
  }
  #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
  pub const fn green_part(&self) -> u16 {
    self.0.y() as u16
  }
  #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
  pub const fn blue_part(&self) -> u16 {
    self.0.z() as u16
  }
}

impl Color for Rgb {
  fn adjust_color_by_percent(&self, percent: StrictPercent) -> Self {
    Self::new([
      percent.as_f32() * self.0.x(),
      percent.as_f32() * self.0.y(),
      percent.as_f32() * self.0.z(),
    ])
    .expect("Adjust color by percent should never cause an invalid color")
  }
  fn blend_two_colors(&self, color_to_blend: &Self, percent_of_initial_color: StrictPercent) -> Self {
    let first_color_adjusted = self.adjust_color_by_percent(percent_of_initial_color);
    let second_color_adjusted =
      color_to_blend.adjust_color_by_percent(StrictPercent::new(1.0).unwrap().overflowing_sub(percent_of_initial_color).0);
    let red_color_part = first_color_adjusted.0.x() + second_color_adjusted.0.x();
    let green_color_part = first_color_adjusted.0.y() + second_color_adjusted.0.y();
    let blue_color_part = first_color_adjusted.0.z() + second_color_adjusted.0.z();
    Rgb::new([red_color_part, green_color_part, blue_color_part])
      .expect("Blending two colors by percent should never be invalid")
  }
}

impl Display for Rgb {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {} {}", self.red_part(), self.green_part(), self.blue_part())
  }
}

#[cfg(test)]
mod test {

  use crate::newtypes::percent::Strict as StrictPercent;

  use super::{Color, Errors as ColorErrors, Rgb};

  #[test]
  fn create_valid_color() {
    let color = Rgb::new([0.0, 255.0, 0.0]).expect("Color should be valid");
    assert_eq!(color.red_part(), 0);
    assert_eq!(color.green_part(), 255);
    assert_eq!(color.blue_part(), 0);
  }
  #[test]
  fn create_valid_color_from_percent() {
    let color = Rgb::new_from_percent([
      StrictPercent::new(0.1).unwrap(),
      StrictPercent::new(0.2).unwrap(),
      StrictPercent::new(0.3).unwrap(),
    ])
    .unwrap();
    assert_eq!(color.red_part(), 25);
    assert_eq!(color.green_part(), 51);
    assert_eq!(color.blue_part(), 76);
  }
  #[test]
  fn create_valid_blue_color_from_percent() {
    let color = Rgb::new_from_percent([
      StrictPercent::new(0.5).unwrap(),
      StrictPercent::new(0.7).unwrap(),
      StrictPercent::new(1.0).unwrap(),
    ])
    .unwrap();
    assert_eq!(color.red_part(), 127);
    assert_eq!(color.green_part(), 179);
    assert_eq!(color.blue_part(), 255);
  }
  #[test]
  fn blend_two_colors() {
    let white = Rgb::white();
    let black = Rgb::black();
    let grey = white.blend_two_colors(&black, StrictPercent::new(0.5).unwrap());
    assert_eq!(grey.red_part(), 127);
    assert_eq!(grey.green_part(), 127);
    assert_eq!(grey.blue_part(), 127);
    let grey = white.blend_two_colors(&black, StrictPercent::new(0.1).unwrap());
    assert_eq!(grey.red_part(), 25);
    assert_eq!(grey.green_part(), 25);
    assert_eq!(grey.blue_part(), 25);
  }
  #[test]
  fn invalid_color_below_is_below_error() {
    let color_result = Rgb::new([-1.0, 55.0, 255.0]);
    assert!(!color_result.is_ok());
    match color_result.unwrap_err() {
      ColorErrors::AboveAndBelow(_) => panic!("This should return a below error, not above and below"),
      ColorErrors::Above(_) => panic!("This should return a below error, not an above error"),
      ColorErrors::Below(_) => (),
    }
  }
  #[test]
  fn invalid_color_below_is_above_error() {
    let color_result = Rgb::new([256.0, 55.0, 0.0]);
    assert!(!color_result.is_ok());
    match color_result.unwrap_err() {
      ColorErrors::AboveAndBelow(_) => panic!("This should return a above error, not above and below"),
      ColorErrors::Above(_) => (),
      ColorErrors::Below(_) => panic!("This should return a above error, not an below error"),
    }
  }
  #[test]
  fn invalid_color_below_is_above_and_below_error() {
    let color_result = Rgb::new([256.0, -1.0, 0.0]);
    assert!(!color_result.is_ok());
    match color_result.unwrap_err() {
      ColorErrors::AboveAndBelow(_) => (),
      ColorErrors::Above(_) => panic!("This should return a above and below error, not just above"),
      ColorErrors::Below(_) => panic!("This should return a above and below error, not just below"),
    }
  }
}
