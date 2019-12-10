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
#![doc(html_favicon_url = "https://raw.githubusercontent.com/kyleu/solitaire/master/crates/assets/embed/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/kyleu/solitaire/master/crates/assets/embed/favicon.png")]
#![doc(issue_tracker_base_url = "https://github.com/kyleu/solitaire/issues/")]

//! `solitaire-controllers` contains actix-web HTTP controllers, usually calling methods from [solitaire-service](solitaire_service).

pub mod admin;
pub mod forms {
  pub mod profile_form;
}
pub mod home;
pub mod routes;
pub mod static_file;
pub mod testbed;
pub mod util {
  pub mod actions;
  pub mod ctx;
  pub mod router;
}
pub mod websocket;
pub mod websocket_msg;

pub(crate) use crate::util::actions::act;
pub(crate) use crate::util::actions::not_found;
pub(crate) use crate::util::actions::redir;
pub(crate) use crate::util::ctx::req_context;
