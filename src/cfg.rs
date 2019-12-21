use solitaire_service::AppConfig;

pub(crate) const DEFAULT_PORT: u16 = 4000;

#[cfg(debug_assertions)]
const fn is_debug() -> bool {
  true
}

#[cfg(not(debug_assertions))]
const fn is_debug() -> bool {
  false
}

pub(crate) fn cfg_from_args() -> AppConfig {
  let matches = crate::args::get_matches();
  let verbose = matches.is_present("verbose") || is_debug();
  let log = crate::log::root_logger(verbose);
  let cfg_dir = match matches.value_of("config") {
    None => default_cfg_dir(&log),
    Some(o) => o.into()
  };
  let (task, address, port) = match matches.subcommand_matches("server") {
    Some(m) => {
      let a = match m.value_of("address") {
        None => "127.0.0.1",
        Some(o) => o
      };
      let p: u16 = match m.value_of("port") {
        Some(p) => match p.parse() {
          Ok(p) => p,
          Err(_) => {
            slog::warn!(log, "Can't parse port number from [{}]", p);
            DEFAULT_PORT
          }
        },
        None => DEFAULT_PORT
      };
      ("server", a, p)
    }
    None => ("app", "127.0.0.1", 0)
  };
  AppConfig::new(task.into(), address.into(), port, &cfg_dir, log, verbose)
}

#[cfg(not(target_os = "android"))]
fn default_cfg_dir(log: &slog::Logger) -> String {
  let app_info = app_dirs2::AppInfo {
    name: solitaire_core::APPNAME,
    author: "Kyle Unverferth <kyle@kyleu.com>"
  };
  let ret: String = match app_dirs2::get_app_root(app_dirs2::AppDataType::UserConfig, &app_info) {
    Ok(d) => d.to_string_lossy().into(),
    Err(e) => {
      slog::warn!(log, "Cannot find default config directory: [{}]", e);
      ".".into()
    }
  };
  ret
}

#[cfg(target_os = "android")]
fn default_cfg_dir(_log: &slog::Logger) -> String {
  format!("/data/user/0/{}/files", solitaire_core::APPNAME)
}
