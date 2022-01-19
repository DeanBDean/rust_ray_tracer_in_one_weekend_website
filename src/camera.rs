use crate::{
  image::Image,
  newtypes::{dimension::Dimension, direction::Direction, distance::Distance, point::Point},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum WidthOrHeight {
  Height(Dimension),
  Width(Dimension),
}

impl Default for WidthOrHeight {
  fn default() -> Self {
    WidthOrHeight::Width(Dimension::from_const(0))
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Camera {
  image: Image,
  view_port_height_or_width: WidthOrHeight,
  focal_length: Distance,
  origin: Point,
}

impl Camera {
  pub const fn _new_from_viewport_width(
    image: Image,
    viewport_width: Dimension,
    focal_length: Distance,
    origin: Point,
  ) -> Self {
    Self {
      image,
      view_port_height_or_width: WidthOrHeight::Width(viewport_width),
      focal_length,
      origin,
    }
  }
  pub const fn new_from_viewport_height(
    image: Image,
    viewport_height: Dimension,
    focal_length: Distance,
    origin: Point,
  ) -> Self {
    Self {
      image,
      view_port_height_or_width: WidthOrHeight::Height(viewport_height),
      focal_length,
      origin,
    }
  }
  pub fn viewport_width(&self) -> Dimension {
    match self.view_port_height_or_width {
      WidthOrHeight::Height(height) => Dimension::from(self.image.aspect_ratio().value() * f32::from(height)),
      WidthOrHeight::Width(width) => width,
    }
  }
  pub fn viewport_height(&self) -> Dimension {
    match self.view_port_height_or_width {
      WidthOrHeight::Height(height) => height,
      WidthOrHeight::Width(width) => Dimension::from(f32::from(width) / self.image.aspect_ratio().value()),
    }
  }
  pub const fn origin(&self) -> Point {
    self.origin
  }
  pub const fn focal_length(&self) -> Distance {
    self.focal_length
  }
  pub fn horizontal(&self) -> Direction {
    Direction::new([f32::from(self.viewport_width()), 0.0, 0.0])
  }
  pub fn vertical(&self) -> Direction {
    Direction::new([0.0, f32::from(self.viewport_height()), 0.0])
  }
  pub fn lower_left_corner(&self) -> Point {
    self.origin()
      - (self.horizontal() / 2.0).into()
      - (self.vertical() / 2.0).into()
      - Point::new([0.0, 0.0, self.focal_length().as_f32()])
  }
}
