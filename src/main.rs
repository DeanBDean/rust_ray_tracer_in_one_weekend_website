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
#![feature(step_trait)]

mod error;
mod newtypes;

use std::io::Write;

use newtypes::dimension::Dimension;

const IMAGE_WIDTH: Dimension = Dimension::from_const(256);
const IMAGE_HEIGHT: Dimension = Dimension::from_const(256);

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn main() {
  print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
  for y_dimension in (Dimension::from(0)..IMAGE_HEIGHT).rev() {
    eprintln!("Scanlines remaining: {}", y_dimension);
    std::io::stderr().flush().expect("Standard error should flush normally");
    for x_dimension in Dimension::from(0)..IMAGE_WIDTH {
      let red_value: f32 = f32::from(x_dimension) / f32::from(IMAGE_WIDTH - Dimension::from(1));
      let green_value: f32 = f32::from(y_dimension) / f32::from(IMAGE_HEIGHT - Dimension::from(1));
      let blue_value: f32 = 0.25;
      let red_value = (255.999 * red_value) as u16;
      let green_value = (255.999 * green_value) as u16;
      let blue_value = (255.999 * blue_value) as u16;

      println!("{} {} {}", red_value, green_value, blue_value);
    }
  }
  eprintln!("Done");
}
