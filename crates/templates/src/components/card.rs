use maud::{html, Markup};
use solitaire_service::RequestContext;

pub(crate) fn card(ctx: &RequestContext, content: &Markup) -> Markup {
  html! {
    div.uk-margin.uk-card.uk-card-body.(ctx.user_profile().theme().card_class()) {
      (content)
    }
  }
}
