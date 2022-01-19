use crate::newtypes::dimension::Dimension;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AspectRatios {
  SixteenByNine,
}

impl Default for AspectRatios {
  fn default() -> Self {
    Self::SixteenByNine
  }
}

impl AspectRatios {
  pub const fn value(self) -> f32 {
    match self {
      Self::SixteenByNine => 16.0 / 9.0,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum WidthOrHeight {
  #[allow(dead_code)]
  Height(Dimension),
  Width(Dimension),
}

impl Default for WidthOrHeight {
  fn default() -> Self {
    Self::Width(Dimension::from(0))
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image {
  aspect_ratio: AspectRatios,
  width_or_height: WidthOrHeight,
}

impl Image {
  pub const fn _new_from_height(aspect_ratio: AspectRatios, height: Dimension) -> Self {
    Self {
      aspect_ratio,
      width_or_height: WidthOrHeight::Height(height),
    }
  }
  pub const fn new_from_width(aspect_ratio: AspectRatios, width: Dimension) -> Self {
    Self {
      aspect_ratio,
      width_or_height: WidthOrHeight::Width(width),
    }
  }
  pub const fn aspect_ratio(&self) -> AspectRatios {
    self.aspect_ratio
  }
  pub fn height(&self) -> Dimension {
    match self.width_or_height {
      WidthOrHeight::Height(height) => height,
      WidthOrHeight::Width(width) => (f32::from(width) / self.aspect_ratio.value()).into(),
    }
  }
  pub fn _width(&self) -> Dimension {
    match self.width_or_height {
      WidthOrHeight::Height(height) => (f32::from(height) * self.aspect_ratio.value()).into(),
      WidthOrHeight::Width(width) => width,
    }
  }
}
