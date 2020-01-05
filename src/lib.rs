#![feature(option_expect_none)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![warn(anonymous_parameters)]
#![warn(bare_trait_objects)]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![warn(missing_docs)]
#![warn(clippy::cargo_common_metadata)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::copy_iterator)]
#![warn(clippy::default_trait_access)]
#![warn(clippy::else_if_without_else)]
#![warn(clippy::empty_line_after_outer_attr)]
#![warn(clippy::exit)]
#![warn(clippy::expl_impl_clone_on_copy)]
#![warn(clippy::explicit_into_iter_loop)]
#![warn(clippy::explicit_iter_loop)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::filter_map)]
#![warn(clippy::filter_map_next)]
#![warn(clippy::find_map)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::if_not_else)]
#![warn(clippy::invalid_upcast_comparisons)]
#![warn(clippy::items_after_statements)]
#![warn(clippy::large_digit_groups)]
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::linkedlist)]
#![warn(clippy::map_flatten)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::maybe_infinite_iter)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::mut_mut)]
#![warn(clippy::mutex_integer)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_continue)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::option_map_unwrap_or)]
#![warn(clippy::option_map_unwrap_or_else)]
#![warn(clippy::option_unwrap_used)]
#![warn(clippy::panic)]
#![warn(clippy::path_buf_push_overwrite)]
// #![warn(clippy::print_stdout)]
#![warn(clippy::pub_enum_variant_names)]
#![warn(clippy::redundant_closure_for_method_calls)]
#![warn(clippy::replace_consts)]
#![warn(clippy::result_map_unwrap_or_else)]
#![warn(clippy::result_unwrap_used)]
#![warn(clippy::same_functions_in_if_condition)]
#![warn(clippy::similar_names)]
#![warn(clippy::todo)]
#![warn(clippy::too_many_lines)]
#![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unreachable)]
#![warn(clippy::unused_self)]
#![warn(clippy::use_debug)]
#![warn(clippy::use_self)]
#![warn(clippy::used_underscore_binding)]
#![warn(clippy::wildcard_dependencies)]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/kyleu/solitaire/master/crates/assets/embed/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/kyleu/solitaire/master/crates/assets/embed/favicon.png")]
#![doc(issue_tracker_base_url = "https://github.com/kyleu/solitaire/issues/")]
#![windows_subsystem = "windows"]

//! `solitaire` is a work in progress. Docs soon.
//! - [solitaire-assets](solitaire_assets)
//! - [solitaire-client](solitaire_client)
//! - [solitaire-controllers](solitaire_controllers)
//! - [solitaire-core](solitaire_core)
//! - [solitaire-service](solitaire_service)
//! - [solitaire-templates](solitaire_templates)

mod app;
mod args;
mod cfg;
mod log;
mod server;

#[cfg(test)]
pub mod tests;

/// Application entrypoint, creates and starts the server
#[actix_rt::main]
pub async fn go() -> anyhow::Result<()> {
  let cfg = crate::cfg::cfg_from_args();
  crate::app::start(cfg).await
}

/// Async application entrypoint, creates and starts the server, returning the port
pub async fn go_async() -> u16 {
  let (port_tx, port_rx) = std::sync::mpsc::channel();
  let cfg = crate::cfg::cfg_from_args();

  let _ = crate::server::start_server(cfg, port_tx).await;

  port_rx.recv().expect("Cannot read port number")
}

/// External app entrypoint, calls `go()` directly and swallows errors
#[no_mangle]
#[allow(clippy::cast_lossless)]
pub extern "C" fn libgo() -> i32 {
  futures::executor::block_on(go_async()) as i32
}

/// Android function
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
  use libc;

  /// JNI entrypoint, calls go()
  #[no_mangle]
  #[allow(unsafe_code)]
  pub unsafe extern "C" fn Java_com_solitaire_App_go() -> libc::c_int {
    crate::go_async() as i32
  }
}
