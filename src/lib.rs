#![feature(option_expect_none)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![warn(anonymous_parameters)]
#![warn(bare_trait_objects)]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_debug_implementations)]
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
pub fn go() -> anyhow::Result<()> {
  let cfg = crate::cfg::cfg_from_args();
  crate::app::start(cfg)
}

/// Async application entrypoint, creates and starts the server, returning the port
pub fn go_async() -> u16 {
  let (port_tx, port_rx) = std::sync::mpsc::channel();
  let cfg = crate::cfg::cfg_from_args();

  let _ = std::thread::spawn(move || {
    match crate::server::start_server(cfg, port_tx) {
      Ok(_) => println!("Successfully started [{}]", solitaire_core::APPNAME),
      Err(e) => println!("Error starting [{}]: {}", solitaire_core::APPNAME, e)
    };
  });

  port_rx.recv().unwrap()
}

/// External app entrypoint, calls `go()` directly and swallows errors
#[no_mangle]
pub extern "C" fn libgo() {
  match go() {
    Ok(_) => println!("Successfully started [{}]", solitaire_core::APPNAME),
    Err(e) => println!("Error starting [{}]: {}", solitaire_core::APPNAME, e)
  };
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
