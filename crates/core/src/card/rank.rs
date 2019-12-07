
use serde::{Deserialize, Serialize};

/// Simple Enum to represent card ranks
#[derive(Debug, Serialize, Deserialize)]
pub enum Rank {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
  Joker,
  Unknown
}

impl Rank {
  pub fn all() -> Vec<Rank> {
    vec!(Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace)
  }

  pub fn to_char(&self) -> char {
    match self {
      Rank::Two => '2',
      Rank::Three => '3',
      Rank::Four => '4',
      Rank::Five => '5',
      Rank::Six => '6',
      Rank::Seven => '7',
      Rank::Eight => '8',
      Rank::Nine => '9',
      Rank::Ten => 'X',
      Rank::Jack => 'J',
      Rank::Queen => 'Q',
      Rank::King => 'K',
      Rank::Ace => 'A',
      Rank::Joker => '!',
      Rank::Unknown => '?'
    }
  }

  pub fn to_index(&self) -> u8 {
    match self {
      Rank::Two => 2,
      Rank::Three => 3,
      Rank::Four => 4,
      Rank::Five => 5,
      Rank::Six => 6,
      Rank::Seven => 7,
      Rank::Eight => 8,
      Rank::Nine => 9,
      Rank::Ten => 10,
      Rank::Jack => 11,
      Rank::Queen => 12,
      Rank::King => 13,
      Rank::Ace => 14,
      Rank::Joker => 0,
      Rank::Unknown => 0
    }
  }
}
