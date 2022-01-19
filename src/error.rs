use std::{error::Error, fmt::Display};

#[allow(clippy::module_name_repetitions)]
pub type LocalError = Box<dyn Error + Send + Sync + 'static>;
#[derive(Debug)]
pub struct DistanceCannotBeNegative;

impl Display for DistanceCannotBeNegative {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Distance cannot be a negative value")
  }
}

impl Error for DistanceCannotBeNegative {}
#[allow(unsafe_code)]
unsafe impl Send for DistanceCannotBeNegative {}
#[allow(unsafe_code)]
unsafe impl Sync for DistanceCannotBeNegative {}
