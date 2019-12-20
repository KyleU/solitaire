use crate::messages::req::GameRequestMessage;

use serde::{Deserialize, Serialize};

/// Class representing a move type
#[derive(Debug, Serialize, Deserialize)]
pub enum PossibleMoveType {
  MoveCards,
  SelectCard,
  SelectPile,
  Undo
}

impl PossibleMoveType {
  pub fn to_char(&self) -> char {
    match self {
      Self::MoveCards => 'm',
      Self::SelectCard => 'c',
      Self::SelectPile => 'p',
      Self::Undo => 'u'
    }
  }
}

/// Class representing a possible move
#[derive(Debug, Serialize, Deserialize)]
pub struct PossibleMove {
  t: PossibleMoveType,
  source_pile: String,
  cards: Vec<u32>,
  target_pile: Option<String>
}

impl PossibleMove {
  pub fn to_message(&self) -> GameRequestMessage {
    match self.t {
      PossibleMoveType::MoveCards => GameRequestMessage::MoveCards {
        cards: self.cards.iter().copied().collect(),
        src: self.source_pile.clone(),
        tgt: self.target_pile.clone().expect("No target pile for MoveCards"),
        auto: true
      },
      PossibleMoveType::SelectCard => GameRequestMessage::SelectCard {
        card: self.cards[0],
        pile: self.source_pile.clone(),
        auto: true
      },
      PossibleMoveType::SelectPile => GameRequestMessage::SelectPile {
        pile: self.source_pile.clone(),
        auto: true
      },
      PossibleMoveType::Undo => GameRequestMessage::Undo
    }
  }
}
