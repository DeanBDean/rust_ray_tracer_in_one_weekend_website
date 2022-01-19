use crate::error::DistanceCannotBeNegative;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Distance(f32);

const fn is_distance_valid(value: f32) -> bool {
  if value < 0.0 {
    return false;
  }
  true
}
impl Distance {
  pub const fn try_from_const(value: f32) -> Result<Self, DistanceCannotBeNegative> {
    if !is_distance_valid(value) {
      return Err(DistanceCannotBeNegative {});
    }
    Ok(Self(value))
  }
  pub const fn as_f32(self) -> f32 {
    self.0
  }
}

impl TryFrom<f32> for Distance {
  type Error = DistanceCannotBeNegative;

  fn try_from(value: f32) -> Result<Self, Self::Error> {
    if !is_distance_valid(value) {
      return Err(DistanceCannotBeNegative {});
    }
    Ok(Self(value))
  }
}

#[cfg(test)]
mod test {
  use super::Distance;
  #[test]
  fn try_from_creates_distance() {
    let distance = Distance::try_from_const(1.0);
    assert!(distance.is_ok());
    let distance = Distance::try_from(1.0);
    assert!(distance.is_ok());
  }
  #[test]
  #[should_panic]
  fn try_from_const_panics_on_negative() {
    Distance::try_from_const(-1.0).expect("This should fail");
  }
  #[test]
  #[should_panic]
  fn try_from_panics_on_negative() {
    Distance::try_from(-1.0).expect("This should fail");
  }
}
