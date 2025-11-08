use dirs::config_dir;
use llm::{
    LLMProvider,
    builder::{LLMBackend, LLMBuilder},
};
use serde::{Deserialize, Serialize};
use std::{fs, sync::Arc};

use crate::persona::Persona;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    api_key: String,
    model: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: "sk-TESTKEY".to_string(),
            model: "google/gemma-3-27b-it".to_string(),
        }
    }
}

impl Settings {
    pub fn new(api_key: String, model: String) -> Self {
        Settings { api_key, model }
    }

    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn model(&self) -> String {
        self.model.clone()
    }

    pub fn llm(&self, char: Arc<Persona>) -> Box<dyn LLMProvider> {
        LLMBuilder::new()
            .backend(LLMBackend::OpenRouter)
            .api_key(self.api_key.clone())
            .model(self.model.clone())
            .system(char.get_description())
            .build()
            .expect("Failed to build LLM (Openrouter)")
    }

    pub fn load() -> Self {
        println!("Loading config started");
        let path = config_dir()
            .map(|mut path| {
                path.push("fullmoon");
                path.push("settings.json");
                path
            })
            .unwrap();

        match path.exists() {
            true => match fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(settings) => {
                        println!("Loading config finished");
                        settings
                    }
                    Err(e) => {
                        eprintln!("Error parsing config: {}", e);
                        Self::default()
                    }
                },
                Err(e) => {
                    eprintln!("Error reading config: {}", e);
                    Self::default()
                }
            },
            false => {
                let default = Self::default();
                println!("Config not found. Writing default");
                default.save().unwrap_or_else(|e| eprintln!("{e}"));
                default
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = config_dir().ok_or("Unable to find config directory")?;

        let fullmoon_dir = config_dir.join("fullmoon");
        if !fullmoon_dir.exists() {
            fs::create_dir_all(&fullmoon_dir)?;
        }

        let config_path = fullmoon_dir.join("settings.json");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;

        Ok(())
    }
}
