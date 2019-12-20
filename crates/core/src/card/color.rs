use serde::{Deserialize, Serialize};

/// Simple Enum to represent card colors
#[derive(Debug, Serialize, Deserialize)]
pub enum Color {
  Red,
  Black,
  Green,
  Blue,
  Colorless,
  Unknown
}

impl Color {
  pub fn to_char(&self) -> char {
    match self {
      Self::Red => 'R',
      Self::Black => 'B',
      Self::Green => 'G',
      Self::Blue => 'U',
      Self::Colorless => 'X',
      Self::Unknown => '?'
    }
  }
}
