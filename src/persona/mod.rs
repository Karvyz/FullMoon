use std::{error::Error, fs, path::PathBuf};

use anyhow::Result;

use crate::persona::{basic::Basic, card::Card};

mod basic;
mod card;
pub mod loader;

#[derive(Debug, Clone)]
pub enum PType {
    Basic(Basic),
    Card(Card),
}

#[derive(Debug, Clone)]
pub struct Persona {
    ptype: PType,
    avatar_uri: Option<String>,
}

impl Persona {
    pub fn new(ptype: PType, avatar_uri: Option<String>) -> Self {
        Persona { ptype, avatar_uri }
    }

    pub fn default_user() -> Self {
        Self {
            ptype: Basic::new("User", ""),
            avatar_uri: None,
        }
    }

    pub fn default_char() -> Self {
        Self {
            ptype: Basic::new("Luna", "You are Luna, an helpfull AI assistant."),
            avatar_uri: None,
        }
    }

    pub fn save(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        let config_path = path.join(format!("{}.json", self.name()));
        let content = match &self.ptype {
            PType::Basic(basic) => serde_json::to_string_pretty(basic)?,
            PType::Card(card) => serde_json::to_string_pretty(card)?,
        };
        fs::write(config_path, content)?;

        Ok(())
    }

    pub fn name(&self) -> String {
        match &self.ptype {
            PType::Basic(basic) => basic.name(),
            PType::Card(card) => card.name(),
        }
    }

    pub fn system_prompt(&self, partner_name: Option<&str>) -> String {
        self.replace_names(
            match &self.ptype {
                PType::Basic(basic) => basic.description(),
                PType::Card(card) => card.persona_prompt(),
            },
            partner_name,
        )
    }

    pub fn greetings(&self, partner_name: Option<&str>) -> Option<Vec<String>> {
        match &self.ptype {
            PType::Basic(_) => None,
            PType::Card(card) => Some(
                card.greetings()
                    .into_iter()
                    .map(|g| self.replace_names(g, partner_name))
                    .collect(),
            ),
        }
    }

    pub fn avatar_uri(&self) -> Option<String> {
        self.avatar_uri.clone()
    }

    pub fn set_avatar_uri(&mut self, path: PathBuf) {
        self.avatar_uri = Some(path.to_str().unwrap().to_string())
    }

    fn replace_names(&self, s: String, partner_name: Option<&str>) -> String {
        let replaced_char_name = s.replace("{{char}}", &self.name());
        match partner_name {
            Some(name) => replaced_char_name.replace("{{user}}", name),
            None => replaced_char_name,
        }
    }
}
