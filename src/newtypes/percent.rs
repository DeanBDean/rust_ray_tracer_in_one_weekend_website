use crate::error::LocalError;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Strict(pub f32);
const MIN_STRICT_VALUE: f32 = 0.0;
const MAX_STRICT_VALUE: f32 = 1.0;

const fn is_strict_percent_valid(value_to_check: f32) -> bool {
  if value_to_check >= MIN_STRICT_VALUE && value_to_check <= MAX_STRICT_VALUE {
    return true;
  }
  false
}

impl Strict {
  pub fn new(value: f32) -> Result<Self, LocalError> {
    if !is_strict_percent_valid(value) {
      return Err("A percent must be between 0.0 and 1.0".into());
    }
    Ok(Self(value))
  }
  pub const fn as_f32(self) -> f32 {
    self.0
  }
  #[allow(dead_code)]
  pub fn overflowing_add(self, right_hand_side: Self) -> (Self, bool) {
    let raw_value_of_add = self.0 + right_hand_side.as_f32();
    if raw_value_of_add > MAX_STRICT_VALUE {
      return (self, false);
    }
    (Strict::new(raw_value_of_add).expect("We know this add is valid"), true)
  }
  pub fn overflowing_sub(self, right_hand_side: Self) -> (Self, bool) {
    let raw_value_of_sub = self.0 - right_hand_side.as_f32();
    if raw_value_of_sub < MIN_STRICT_VALUE {
      return (self, false);
    }
    (Strict::new(raw_value_of_sub).expect("We know this sub is valid"), true)
  }
}

#[cfg(test)]
mod test {
  use super::Strict;

  #[test]
  fn overflowing_add() {
    assert_eq!(
      Strict::new(0.8)
        .expect("this is valid")
        .overflowing_add(Strict::new(0.21).expect("this is valid")),
      (Strict::new(0.8).expect("this is valid"), false)
    );
    assert_eq!(
      Strict::new(0.8)
        .expect("this is valid")
        .overflowing_add(Strict::new(0.19).expect("this is valid")),
      (Strict::new(0.99).expect("this is valid"), true)
    );
  }
  #[test]
  fn overflowing_sub() {
    assert_eq!(
      Strict::new(0.8)
        .expect("this is valid")
        .overflowing_sub(Strict::new(0.81).expect("this is valid")),
      (Strict::new(0.8).expect("this is valid"), false)
    );
    assert_eq!(
      Strict::new(0.8)
        .expect("this is valid")
        .overflowing_sub(Strict::new(0.79).expect("this is valid")),
      (Strict::new(0.8 - 0.79).expect("this is valid"), true)
    );
  }
}
