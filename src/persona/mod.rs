use std::{error::Error, fs, path::PathBuf, time::SystemTime};

use crate::persona::{basic::Basic, card::Card};

mod basic;
mod card;

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

    pub fn description(&self) -> String {
        match &self.ptype {
            PType::Basic(basic) => basic.description(),
            PType::Card(card) => card.description(),
        }
    }

    pub fn system_prompt(&self) -> String {
        match &self.ptype {
            PType::Basic(basic) => basic.description(),
            PType::Card(card) => card.persona_prompt(),
        }
    }

    pub fn greetings(&self) -> Option<Vec<String>> {
        match &self.ptype {
            PType::Basic(_) => None,
            PType::Card(card) => Some(card.greetings()),
        }
    }

    pub fn avatar_uri(&self) -> Option<String> {
        self.avatar_uri.clone()
    }
}

pub struct PersonaLoader {}

impl PersonaLoader {
    pub fn load_from_cache(subdir: &str) -> Vec<Persona> {
        let dir = Self::cache_path(subdir);
        let mut personas = vec![];

        match fs::read_dir(&dir) {
            Err(_) => {
                println!("Cache not found. Writing default");
                let char = Persona::default_char();
                if char.save(dir).is_err() {
                    eprintln!("Writing default failed");
                }
                personas.push(char);
            }
            Ok(entries) => {
                for entry in entries {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if let Ok(persona) = Self::load(path) {
                        personas.push(persona);
                    }
                }
            }
        }
        personas
    }

    pub fn load_most_recent_from_cache(subdir: &str) -> Option<Persona> {
        let path = Self::cache_path(subdir);
        let mut most_recent_file: Option<(PathBuf, SystemTime)> = None;

        // Read directory contents and find the most recent file
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            // Check if it's a file (not a directory)
            if path.is_file() {
                let modified_time = Self::modified_time(&path);
                match most_recent_file {
                    None => most_recent_file = Some((path, modified_time)),
                    Some((_, current_time)) => {
                        if modified_time > current_time {
                            most_recent_file = Some((path, modified_time));
                        }
                    }
                }
            }
        }

        match most_recent_file {
            None => None,
            Some((path, _)) => Self::load(path).ok(),
        }
    }

    fn load(path: PathBuf) -> Result<Persona, Box<dyn Error>> {
        let data = fs::read_to_string(&path)?;
        if let Ok(card) = Card::load_from_json(&data) {
            let persona = Persona::new(card.into(), None);
            println!("Loaded card {}", persona.name());
            return Ok(persona);
        }

        let basic = Basic::load_from_json(&data)?;
        let persona = Persona::new(basic.into(), None);
        println!("Loaded simple {}", persona.name());
        Ok(persona)
    }

    fn cache_path(subdir: &str) -> PathBuf {
        dirs::cache_dir()
            .map(|mut path| {
                path.push("fullmoon");
                path.push(subdir);
                path
            })
            .unwrap()
    }

    fn modified_time(path: &PathBuf) -> SystemTime {
        if let Ok(metadata) = fs::metadata(path)
            && let Ok(modified_time) = metadata.modified()
        {
            return modified_time;
        }
        SystemTime::UNIX_EPOCH
    }
}
