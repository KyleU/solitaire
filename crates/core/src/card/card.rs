use crate::card::rank::Rank;
use crate::card::suit::Suit;

use serde::{Deserialize, Serialize};

/// Class representing a single card
#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
  id: u32,
  r: Rank,
  s: Suit,
  u: bool
}

impl ToString for Card {
  fn to_string(&self) -> String {
    format!(
      "{}:{}{}{}",
      self.id,
      self.r.to_char(),
      self.s.to_char(),
      if self.u { "+" } else { "-" }
    )
  }
}
