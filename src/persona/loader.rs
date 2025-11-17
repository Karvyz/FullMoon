use anyhow::{Result, anyhow};
use std::{fs, path::PathBuf, time::SystemTime};

use crate::persona::{Persona, basic::Basic, card::Card};

pub enum Subdir {
    Chars,
    Users,
}

impl Subdir {
    fn default_persona(&self) -> Persona {
        match self {
            Subdir::Chars => Persona::default_char(),
            Subdir::Users => Persona::default_user(),
        }
    }
}

pub struct PersonaLoader {}

impl PersonaLoader {
    pub fn load_from_cache(subdir: Subdir) -> Vec<Persona> {
        let dir = Self::cache_path(&subdir);
        match Self::try_load_dir(dir) {
            Ok(personas) => personas,
            Err(e) => {
                eprintln!("{e}");
                vec![subdir.default_persona()]
            }
        }
    }

    pub fn load_most_recent_from_cache(subdir: Subdir) -> Persona {
        let cache_path = Self::cache_path(&subdir);
        match Self::most_recent_dir(&cache_path) {
            Ok(most_recent) => match Self::try_load_subdir(most_recent) {
                Ok(persona) => return persona,
                Err(e) => eprintln!("{e}"),
            },

            Err(e) => eprintln!("{e}"),
        }
        subdir.default_persona()
    }

    fn most_recent_dir(path: &PathBuf) -> Result<PathBuf> {
        let mut most_recent_dir: Result<PathBuf> = Err(anyhow!("No file found"));
        let mut most_recent_change = SystemTime::UNIX_EPOCH;
        for entry in (fs::read_dir(path)?).flatten() {
            let path = entry.path();

            if path.is_dir() {
                let modified_time = Self::modified_time(&path);
                if modified_time > most_recent_change {
                    most_recent_change = modified_time;
                    most_recent_dir = Ok(path)
                }
            }
        }
        most_recent_dir
    }

    fn try_load_dir(dir: PathBuf) -> Result<Vec<Persona>> {
        let mut personas = vec![];
        for entry in (fs::read_dir(dir)?).flatten() {
            let path = entry.path();
            if path.is_dir()
                && let Ok(persona) = Self::try_load_subdir(path)
            {
                personas.push(persona);
            }
        }
        Ok(personas)
    }

    fn try_load_subdir(dir: PathBuf) -> Result<Persona> {
        let mut image = None;
        let mut persona: Result<Persona> = Err(anyhow!("Persona not found"));
        for entry in (fs::read_dir(dir)?).flatten() {
            let path = entry.path();
            if path.is_file()
                && let Some(ext) = path.extension()
                && let Some(ext) = ext.to_str()
            {
                match ext {
                    "json" => persona = Self::load(path),
                    "png" => image = Some(path),
                    _ => (),
                }
            }
        }

        match persona {
            Ok(mut persona) => {
                if let Some(image) = image {
                    persona.set_avatar_uri(image);
                }
                Ok(persona)
            }
            Err(_) => Err(anyhow!("Persona not found")),
        }
    }

    fn load(path: PathBuf) -> Result<Persona> {
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

    fn cache_path(subdir: &Subdir) -> PathBuf {
        dirs::cache_dir()
            .map(|mut path| {
                path.push("fullmoon");
                path.push(match subdir {
                    Subdir::Chars => "chars",
                    Subdir::Users => "users",
                });
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
