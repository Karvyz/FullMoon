use serde::{Deserialize, Serialize};

use crate::persona::PType;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Card {}

impl From<Card> for PType {
    fn from(card: Card) -> Self {
        PType::Card(card)
    }
}

impl Card {
    pub fn load_from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }
}
