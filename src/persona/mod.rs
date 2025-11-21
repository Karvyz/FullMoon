use std::{fmt::Debug, ops::Deref, rc::Rc};

use iced::widget::{Image, image::Handle};

use crate::persona::basic::Basic;

mod basic;
mod card;
pub mod loader;

pub trait CharData {
    fn name(&self) -> &str;
    fn system_prompt(&self, partner_name: Option<&str>) -> String;
    fn greetings(&self, partner_name: Option<&str>) -> Option<Vec<String>>;
}

#[derive(Clone)]
pub struct Persona {
    data: Rc<dyn CharData>,
    image: Handle,
}

impl Debug for Persona {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Persona")
            .field("char", &self.data.name())
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
    pub fn new(data: Rc<dyn CharData>, image: Handle) -> Self {
        Persona { data, image }
    }

    pub fn default_user() -> Self {
        Self {
            data: Basic::new("User", ""),
            image: Handle::from_path("assets/user.png"),
        }
    }

    pub fn default_char() -> Self {
        Self {
            data: Basic::new("Luna", "You are Luna, an helpfull AI assistant."),
            image: Handle::from_path("assets/char.png"),
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

    pub fn image(&self) -> Image {
        iced::widget::image(&self.image)
    }

    pub fn replace_names(s: &str, self_name: &str, partner_name: Option<&str>) -> String {
        let replaced_char_name = s.replace("{{char}}", self_name);
        match partner_name {
            Some(name) => replaced_char_name.replace("{{user}}", name),
            None => replaced_char_name,
        }
    }
}
