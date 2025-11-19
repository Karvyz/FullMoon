use dirs::config_dir;
use llm::{
    LLMProvider,
    builder::{LLMBackend, LLMBuilder},
};
use log::{error, trace};
use serde::{Deserialize, Serialize};
use std::{fs, sync::Arc};

use crate::persona::Persona;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
    reasoning: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: "sk-TESTKEY".to_string(),
            model: "google/gemma-3-27b-it".to_string(),
            temperature: 0.5,
            max_tokens: 1000,
            reasoning: false,
        }
    }
}

impl Settings {
    pub fn new(
        api_key: String,
        model: String,
        temperature: f32,
        max_tokens: u32,
        reasoning: bool,
    ) -> Self {
        Settings {
            api_key,
            model,
            temperature,
            max_tokens,
            reasoning,
        }
    }

    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn model(&self) -> String {
        self.model.clone()
    }

    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    pub fn max_tokens(&self) -> u32 {
        self.max_tokens
    }

    pub fn reasoning(&self) -> bool {
        self.reasoning
    }

    pub fn llm(&self, char: &Arc<Persona>, user: &Arc<Persona>) -> Box<dyn LLMProvider> {
        LLMBuilder::new()
            .backend(LLMBackend::OpenRouter)
            .api_key(self.api_key.clone())
            .model(self.model.clone())
            .temperature(self.temperature)
            .max_tokens(self.max_tokens)
            .reasoning(self.reasoning)
            .system(char.system_prompt(Some(&user.name())))
            .build()
            .expect("Failed to build LLM (Openrouter)")
    }

    pub fn load() -> Self {
        trace!("Loading config started");
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
                        trace!("Loading config finished");
                        settings
                    }
                    Err(e) => {
                        error!("Error parsing config: {}", e);
                        Self::default()
                    }
                },
                Err(e) => {
                    error!("Error reading config: {}", e);
                    Self::default()
                }
            },
            false => {
                let default = Self::default();
                error!("Config not found. Writing default");
                default.save().unwrap_or_else(|e| error!("{e}"));
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
