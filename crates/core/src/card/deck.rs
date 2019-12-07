use crate::card::card::Card;
use crate::card::rank::Rank;

use serde::{Deserialize, Serialize};

/// Class representing a collection of cards
#[derive(Debug, Serialize, Deserialize)]
struct Deck {
  cards: Vec<Card>,
  low_rank: Rank,
  high_rank: Rank,
  original_order: Vec<u32>
}

impl ToString for Deck {
  fn to_string(&self) -> String {
    format!("")
  }
}
