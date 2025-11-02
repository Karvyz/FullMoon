use std::sync::Arc;

use crate::persona::Persona;
use llm::chat::ChatMessage;

#[derive(Clone)]
pub struct Message {
    pub owner: Arc<dyn Persona>,
    pub text: String,
}

impl Message {
    pub fn new(owner: Arc<dyn Persona>, text: String) -> Self {
        Message {
            owner,
            text: text.trim().to_string(),
        }
    }

    pub fn empty(owner: Arc<dyn Persona>) -> Self {
        Message {
            owner,
            text: String::new(),
        }
    }

    pub fn to_chat_message(&self) -> ChatMessage {
        self.owner.create_message(self.text.clone())
    }
}
