use crate::server::start_server;

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub(crate) fn start(cfg: solitaire_service::AppConfig) -> anyhow::Result<()> {
  let (port_tx, port_rx) = std::sync::mpsc::channel();
  match cfg.task().as_ref() {
    "app" => {
      let _ = std::thread::spawn(move || start_server(cfg, port_tx));
      let port = port_rx.recv().unwrap();
      web_view::builder()
        .title(solitaire_core::APPNAME)
        .content(web_view::Content::Url(format!("http://127.0.0.1:{}", port)))
        .size(1920, 1080)
        .resizable(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .map_err(|e| anyhow::anyhow!("Error creating webview: {:?}", e))
    }
    _ => start_server(cfg, port_tx)
  }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub(crate) fn start(cfg: solitaire_service::AppConfig) -> anyhow::Result<()> {
  let (port_tx, _) = std::sync::mpsc::channel();
  start_server(cfg, port_tx)
}
