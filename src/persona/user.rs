use llm::chat::ChatMessage;

use crate::persona::Persona;

pub struct User {
    name: String,
    description: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "User".to_string(),
            description: String::new(),
        }
    }
}

impl Persona for User {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_avatar_uri(&self) -> String {
        "assets/user.png".to_string()
    }

    fn create_message(&self, text: String) -> ChatMessage {
        ChatMessage::user().content(text).build()
    }
}
