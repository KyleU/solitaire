use crate::ctx::ClientContext;

use anyhow::Result;
use maud::html;
use solitaire_core::profile::UserProfile;
use solitaire_core::ResponseMessage;
use std::sync::RwLock;
use uuid::Uuid;

pub(crate) struct MessageHandler {}

impl MessageHandler {
  pub(crate) fn handle(ctx: &RwLock<ClientContext>, msg: ResponseMessage) -> Result<()> {
    debug!("Message received: {:?}", msg);
    match msg {
      ResponseMessage::Connected {
        connection_id,
        user_id,
        u,
        b
      } => on_connected(ctx, connection_id, user_id, &u, b),
      ResponseMessage::Pong { v } => on_pong(ctx, v),
      ResponseMessage::Notification { level, content } => crate::logging::notify(level, &content),

      // Custom Messages
      _ => {
        warn!("Unhandled ResponseMessage [{:?}]", msg);
        Ok(())
      }
    }
  }
}

fn on_connected(ctx: &RwLock<ClientContext>, connection_id: Uuid, user_id: Uuid, u: &UserProfile, b: bool) -> Result<()> {
  ctx.write().unwrap().on_connected(connection_id, user_id, u.clone(), b);
  let c = ctx.read().unwrap();
  let _ = c.append_template(
    "socket-results",
    "div",
    html!((format!("Connect message received, {} connection", if b { "binary" } else { "text" })))
  )?;
  Ok(())
}

fn on_pong(ctx: &RwLock<ClientContext>, v: i64) -> Result<()> {
  let now = js_sys::Date::now() as i64;
  let msg = format!("Pong: [{}ms] elapsed", now - v);
  let _ = ctx.read().unwrap().append_template("socket-results", "div", html!((msg)))?;
  info!("{}", msg);
  Ok(())
}
