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
  pub fn standard() -> Vec<Self> {
    vec![Self::Hearts, Self::Spades, Self::Diamonds, Self::Clubs]
  }

  pub fn all() -> Vec<Self> {
    vec![
      Self::Hearts,
      Self::Spades,
      Self::Diamonds,
      Self::Clubs,
      Self::Horseshoes,
      Self::Stars,
      Self::Tridents,
      Self::Moons,
    ]
  }

  pub fn to_char(&self) -> char {
    match self {
      Self::Hearts => 'H',
      Self::Spades => 'S',
      Self::Diamonds => 'D',
      Self::Clubs => 'C',
      Self::Horseshoes => 'O',
      Self::Stars => 'R',
      Self::Tridents => 'T',
      Self::Moons => 'M',
      Self::Suitless => 'X',
      Self::Unknown => '?'
    }
  }

  pub fn color(&self) -> Color {
    match self {
      Self::Hearts | Self::Diamonds | Self::Stars => Color::Red,
      Self::Spades | Self::Clubs | Self::Horseshoes => Color::Black,
      Self::Tridents => Color::Green,
      Self::Moons => Color::Blue,
      Self::Suitless => Color::Colorless,
      Self::Unknown => Color::Unknown
    }
  }
}
