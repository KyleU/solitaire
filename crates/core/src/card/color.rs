use serde::{Deserialize, Serialize};

/// Simple Enum to represent card colors
#[derive(Debug, Serialize, Deserialize)]
pub enum Color {
  Red,
  Black,
  Green,
  Blue,
  Colorless,
  UnknownColor
}

impl Color {
  pub fn to_char(&self) -> char {
    match self {
      Color::Red => 'R',
      Color::Black => 'B',
      Color::Green => 'G',
      Color::Blue => 'U',
      Color::Colorless => 'X',
      Color::UnknownColor => '?'
    }
  }
}
