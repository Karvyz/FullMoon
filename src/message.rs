use std::sync::Arc;

use crate::persona::Persona;
use llm::chat::ChatMessage;

#[derive(Clone)]
pub enum OwnerType {
    User,
    Char,
}

#[derive(Clone)]
pub struct Message {
    pub owner: Arc<Persona>,
    pub owner_type: OwnerType,
    pub text: String,
}

impl Message {
    pub fn from_user(user: Arc<Persona>, text: String) -> Self {
        Message {
            owner: user,
            owner_type: OwnerType::User,
            text,
        }
    }

    pub fn from_char(char: Arc<Persona>, text: String) -> Self {
        Message {
            owner: char,
            owner_type: OwnerType::Char,
            text: text.trim().to_string(),
        }
    }

    pub fn empty_from_char(char: Arc<Persona>) -> Self {
        Self::from_char(char, String::new())
    }

    pub fn to_chat_message(&self) -> ChatMessage {
        match self.owner_type {
            OwnerType::User => ChatMessage::user().content(&self.text).build(),
            OwnerType::Char => ChatMessage::assistant().content(&self.text).build(),
        }
    }

    pub fn get_avatar_uri(&self) -> String {
        match self.owner.avatar_uri() {
            Some(uri) => uri,
            None => match self.owner_type {
                OwnerType::User => "assets/user.png".to_string(),
                OwnerType::Char => "assets/char.png".to_string(),
            },
        }
    }
}
