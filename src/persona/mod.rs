use std::{fmt::Debug, ops::Deref, path::PathBuf, rc::Rc};

use crate::persona::basic::Basic;

mod basic;
mod card;
pub mod loader;

pub trait CharData {
    fn name(&self) -> String;
    fn system_prompt(&self, partner_name: Option<&str>) -> String;
    fn greetings(&self, partner_name: Option<&str>) -> Option<Vec<String>>;
}

#[derive(Clone)]
pub struct Persona {
    avatar_uri: Option<String>,
    data: Rc<dyn CharData>,
}

impl Debug for Persona {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Persona")
            .field("avatar_uri", &self.avatar_uri)
            // .field("char_data", &self.char_data)
            .finish()
    }
}

impl Deref for Persona {
    type Target = Rc<dyn CharData>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Persona {
    pub fn new(data: Rc<dyn CharData>, avatar_uri: Option<String>) -> Self {
        Persona { data, avatar_uri }
    }

    pub fn default_user() -> Self {
        Self {
            data: Basic::new("User", ""),
            avatar_uri: None,
        }
    }

    pub fn default_char() -> Self {
        Self {
            data: Basic::new("Luna", "You are Luna, an helpfull AI assistant."),
            avatar_uri: None,
        }
    }

    // pub fn save(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
    //     if !path.exists() {
    //         fs::create_dir_all(&path)?;
    //     }
    //
    //     let config_path = path.join(format!("{}.json", self.name()));
    //     let content = match &self.ptype {
    //         PType::Basic(basic) => serde_json::to_string_pretty(basic)?,
    //         PType::Card(card) => serde_json::to_string_pretty(card)?,
    //     };
    //     fs::write(config_path, content)?;
    //
    //     Ok(())
    // }

    pub fn avatar_uri(&self) -> Option<String> {
        self.avatar_uri.clone()
    }

    pub fn set_avatar_uri(&mut self, path: PathBuf) {
        self.avatar_uri = Some(path.to_str().unwrap().to_string())
    }

    pub fn replace_names(s: &str, self_name: &str, partner_name: Option<&str>) -> String {
        let replaced_char_name = s.replace("{{char}}", self_name);
        match partner_name {
            Some(name) => replaced_char_name.replace("{{user}}", name),
            None => replaced_char_name,
        }
    }
}
