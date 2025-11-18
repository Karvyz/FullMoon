use std::sync::Arc;

use crate::persona::Persona;
use iced::widget::text_editor::Content;
use llm::chat::ChatMessage;

#[derive(Clone)]
pub enum OwnerType {
    User,
    Char,
}

pub struct Message {
    pub owner: Arc<Persona>,
    pub owner_type: OwnerType,
    pub text: String,
    pub editing: Option<Content>,
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Self {
            owner: self.owner.clone(),
            owner_type: self.owner_type.clone(),
            text: self.text.clone(),
            editing: None,
        }
    }
}

impl Message {
    pub fn from_user(user: Arc<Persona>, text: String) -> Self {
        Message {
            owner: user,
            owner_type: OwnerType::User,
            text,
            editing: None,
        }
    }

    pub fn from_char(char: Arc<Persona>, text: String) -> Self {
        Message {
            owner: char,
            owner_type: OwnerType::Char,
            text: text.trim().to_string(),
            editing: None,
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
