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
  pub fn all() -> Vec<Self> {
    vec![
      Self::Two,
      Self::Three,
      Self::Four,
      Self::Five,
      Self::Six,
      Self::Seven,
      Self::Eight,
      Self::Nine,
      Self::Ten,
      Self::Jack,
      Self::Queen,
      Self::King,
      Self::Ace,
    ]
  }

  pub fn to_char(&self) -> char {
    match self {
      Self::Two => '2',
      Self::Three => '3',
      Self::Four => '4',
      Self::Five => '5',
      Self::Six => '6',
      Self::Seven => '7',
      Self::Eight => '8',
      Self::Nine => '9',
      Self::Ten => 'X',
      Self::Jack => 'J',
      Self::Queen => 'Q',
      Self::King => 'K',
      Self::Ace => 'A',
      Self::Joker => '!',
      Self::Unknown => '?'
    }
  }

  pub fn to_index(&self) -> u8 {
    match self {
      Self::Two => 2,
      Self::Three => 3,
      Self::Four => 4,
      Self::Five => 5,
      Self::Six => 6,
      Self::Seven => 7,
      Self::Eight => 8,
      Self::Nine => 9,
      Self::Ten => 10,
      Self::Jack => 11,
      Self::Queen => 12,
      Self::King => 13,
      Self::Ace => 14,
      Self::Joker | Self::Unknown => 0
    }
  }
}
