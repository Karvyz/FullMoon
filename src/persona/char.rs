use std::{error::Error, fs, path::PathBuf};

use llm::chat::ChatMessage;
use serde::{Deserialize, Serialize};

use crate::persona::Persona;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Char {
    name: String,
    description: String,
}

impl Char {
    pub fn load_from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }

    pub fn save(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        let config_path = path.join(format!("{}.json", self.name));
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;

        Ok(())
    }
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
