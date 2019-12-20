use crate::card::card::Card;
use crate::game::moves::PossibleMove;
use crate::util::NotificationLevel;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Sent from server to client, this shared model is used for all client communication
#[allow(variant_size_differences)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ResponseMessage {
  Connected {
    connection_id: Uuid,
    user_id: Uuid,
    u: Box<crate::profile::UserProfile>,
    b: bool
  },
  ServerError {
    reason: String,
    content: String
  },
  Pong {
    v: i64
  },
  Notification {
    level: NotificationLevel,
    content: String
  }
}

impl ResponseMessage {
  pub fn from_json(s: &str) -> Result<Self> {
    serde_json::from_str(s).with_context(|| "Can't decode json ResponseMessage")
  }

  pub fn to_json(&self) -> Result<String> {
    serde_json::to_string_pretty(&self).with_context(|| "Can't encode json ResponseMessage")
  }

  pub fn from_binary(b: &[u8]) -> Result<Self> {
    bincode::deserialize(b).with_context(|| "Can't decode binary ResponseMessage")
  }

  pub fn to_binary(&self) -> Result<Vec<u8>> {
    bincode::serialize(&self).with_context(|| "Can't encode binary ResponseMessage")
  }
}

/// Game response message
#[derive(Debug, Serialize, Deserialize)]
pub enum GameResponseMessage {
  PossibleMoves {
    moves: Vec<PossibleMove>,
    undos_available: u16,
    redos_available: u16
  },
  CardRevealed {
    card: Card
  },
  CardHidden {
    id: u32
  },
  CardMoveCancelled {
    cards: Vec<u32>,
    source: String
  },
  CardsMoved {
    cards: Vec<u32>,
    source: String,
    target: String,
    turn: Option<bool>
  },
  MessageSet {
    messages: Box<Vec<ResponseMessage>>
  }
}
