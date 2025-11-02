use llm::chat::ChatMessage;

use crate::persona::Persona;

pub struct Char {
    name: String,
    description: String,
}

impl Default for Char {
    fn default() -> Self {
        Self {
            name: "Luna".to_string(),
            description: "You are Luna, an helpfull AI assistant.".to_string(),
        }
    }
}

impl Persona for Char {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_avatar_uri(&self) -> String {
        "assets/char.png".to_string()
    }

    fn create_message(&self, text: String) -> ChatMessage {
        ChatMessage::assistant().content(text).build()
    }
}
