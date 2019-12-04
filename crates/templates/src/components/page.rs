use crate::components;

use anyhow::Result;
use maud::{html, Markup, PreEscaped};
use solitaire_service::{RequestContext, Router};

pub(crate) fn page(ctx: &RequestContext, router: &dyn Router, title: &str, content: Markup) -> Result<Markup> {
  Ok(html! {
    (PreEscaped("<!DOCTYPE html>"))
    html lang="en" {
      (components::header::header(&ctx, router, &format!("{} - {}", title, solitaire_core::APPNAME))?)
      body.(ctx.user_profile().theme().body_class()) {
        (content)
      }
    }
  })
}
