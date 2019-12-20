use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use solitaire_service::AppConfig;

/// Available at `/testbed/{key}`
pub fn testbed_key(session: Session, cfg: web::Data<AppConfig>, key: web::Path<String>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    let k: &str = &key;
    match k {
      "dump" => solitaire_templates::testbed::dump(ctx, router),
      "gallery" => solitaire_templates::testbed::gallery(ctx, router),
      "prototype" => solitaire_templates::testbed::prototype(ctx, router),
      "scroll" => solitaire_templates::testbed::scroll(ctx, router),
      _ => Err(anyhow::anyhow!("Cannot find testbed matching [{}]", key))
    }
  })
}
