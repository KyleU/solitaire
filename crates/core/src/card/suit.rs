use crate::card::color::Color;

use serde::{Deserialize, Serialize};

/// Simple Enum to represent card suits
#[derive(Debug, Serialize, Deserialize)]
pub enum Suit {
  Hearts,
  Spades,
  Diamonds,
  Clubs,
  Horseshoes,
  Stars,
  Tridents,
  Moons,
  Suitless,
  Unknown
}

impl Suit {
  pub fn standard() -> Vec<Suit> {
    vec![Suit::Hearts, Suit::Spades, Suit::Diamonds, Suit::Clubs]
  }

  pub fn all() -> Vec<Suit> {
    vec![
      Suit::Hearts,
      Suit::Spades,
      Suit::Diamonds,
      Suit::Clubs,
      Suit::Horseshoes,
      Suit::Stars,
      Suit::Tridents,
      Suit::Moons,
    ]
  }

  pub fn to_char(&self) -> char {
    match self {
      Suit::Hearts => 'H',
      Suit::Spades => 'S',
      Suit::Diamonds => 'D',
      Suit::Clubs => 'C',
      Suit::Horseshoes => 'O',
      Suit::Stars => 'R',
      Suit::Tridents => 'T',
      Suit::Moons => 'M',
      Suit::Suitless => 'X',
      Suit::Unknown => '?'
    }
  }

  pub fn color(&self) -> Color {
    match self {
      Suit::Hearts => Color::Red,
      Suit::Spades => Color::Black,
      Suit::Diamonds => Color::Red,
      Suit::Clubs => Color::Black,
      Suit::Horseshoes => Color::Black,
      Suit::Stars => Color::Red,
      Suit::Tridents => Color::Green,
      Suit::Moons => Color::Blue,
      Suit::Suitless => Color::Colorless,
      Suit::Unknown => Color::UnknownColor
    }
  }
}
