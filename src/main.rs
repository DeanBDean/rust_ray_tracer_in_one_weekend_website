#![deny(unsafe_code)]
#![warn(
  clippy::all,
  clippy::await_holding_lock,
  clippy::char_lit_as_u8,
  clippy::checked_conversions,
  clippy::dbg_macro,
  clippy::debug_assert_with_mut_call,
  clippy::doc_markdown,
  clippy::empty_enum,
  clippy::enum_glob_use,
  clippy::exit,
  clippy::expl_impl_clone_on_copy,
  clippy::explicit_deref_methods,
  clippy::explicit_into_iter_loop,
  clippy::fallible_impl_from,
  clippy::filter_map_next,
  clippy::float_cmp_const,
  clippy::fn_params_excessive_bools,
  clippy::if_let_mutex,
  clippy::implicit_clone,
  clippy::imprecise_flops,
  clippy::inefficient_to_string,
  clippy::invalid_upcast_comparisons,
  clippy::large_types_passed_by_value,
  clippy::let_unit_value,
  clippy::linkedlist,
  clippy::lossy_float_literal,
  clippy::macro_use_imports,
  clippy::manual_ok_or,
  clippy::map_err_ignore,
  clippy::map_flatten,
  clippy::map_unwrap_or,
  clippy::match_on_vec_items,
  clippy::match_same_arms,
  clippy::match_wildcard_for_single_variants,
  clippy::mem_forget,
  clippy::mismatched_target_os,
  clippy::mut_mut,
  clippy::mutex_integer,
  clippy::needless_borrow,
  clippy::needless_continue,
  clippy::option_option,
  clippy::path_buf_push_overwrite,
  clippy::ptr_as_ptr,
  clippy::ref_option_ref,
  clippy::rest_pat_in_fully_bound_structs,
  clippy::same_functions_in_if_condition,
  clippy::semicolon_if_nothing_returned,
  clippy::string_add_assign,
  clippy::string_add,
  clippy::string_lit_as_bytes,
  clippy::string_to_string,
  clippy::todo,
  clippy::trait_duplication_in_bounds,
  clippy::unimplemented,
  clippy::unnested_or_patterns,
  clippy::unused_self,
  clippy::useless_transmute,
  clippy::verbose_file_reads,
  clippy::zero_sized_map_values,
  future_incompatible,
  nonstandard_style,
  rust_2018_idioms
)]
#![warn(clippy::pedantic)]
#![feature(const_fn_floating_point_arithmetic, const_fn_trait_bound, step_trait)]

mod camera;
mod error;
mod image;
mod newtypes;
mod ray;
mod vec3;

use std::io::Write;

use newtypes::dimension::Dimension;

use crate::{
  camera::Camera,
  image::{AspectRatios, Image},
  newtypes::{distance::Distance, point::Point},
  ray::Ray,
};

const IMAGE_WIDTH: Dimension = Dimension::from_const(256);

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn main() {
  let image = Image::new_from_width(AspectRatios::SixteenByNine, IMAGE_WIDTH);
  let image_height = image.height();
  let camera = Camera::new_from_viewport_height(
    image,
    Dimension::from_const(2),
    Distance::try_from_const(1.0).expect("This is positive and valid"),
    Point::new([0.0, 0.0, 0.0]),
  );
  let origin = camera.origin();
  let horizontal = camera.horizontal();
  let vertical = camera.vertical();
  let lower_left_corner = camera.lower_left_corner();
  print!("P3\n{} {}\n255\n", IMAGE_WIDTH, image_height);
  for y_dimension in (Dimension::from(0)..image_height).rev() {
    eprintln!("Scanlines remaining: {}", y_dimension);
    std::io::stderr().flush().expect("Standard error should flush normally");
    let v = f32::from(y_dimension) / f32::from(image_height - Dimension::from_const(1));
    for x_dimension in Dimension::from(0)..IMAGE_WIDTH {
      let u = f32::from(x_dimension) / f32::from(IMAGE_WIDTH - Dimension::from_const(1));
      let ray = Ray::new(
        origin,
        (lower_left_corner + (u * horizontal).into() + (v * vertical).into() - origin).into(),
      );
      let color = ray.find_color();

      println!("{}", color);
    }
  }
  eprintln!("Done");
}
