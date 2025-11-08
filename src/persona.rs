use std::{error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Persona {
    name: String,
    description: String,
    avatar_uri: Option<String>,
}

impl Persona {
    pub fn default_user() -> Self {
        Self {
            name: "User".to_string(),
            description: String::new(),
            avatar_uri: None,
        }
    }

    pub fn default_char() -> Self {
        Self {
            name: "Luna".to_string(),
            description: "You are Luna, an helpfull AI assistant.".to_string(),
            avatar_uri: None,
        }
    }

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

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_avatar_uri(&self) -> Option<String> {
        self.avatar_uri.clone()
    }
}

// impl Default for Persona {
//     fn default() -> Self {
//         Self {
//             name: "Luna".to_string(),
//             description: "You are Luna, an helpfull AI assistant.".to_string(),
//         }
//     }
// }
