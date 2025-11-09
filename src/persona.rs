use std::{error::Error, fs, path::PathBuf, time::SystemTime};

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
        match fs::read_to_string(&path) {
            Ok(data) => match Persona::load_from_json(&data) {
                Ok(persona) => {
                    println!("Loaded {}", persona.get_name());
                    Ok(persona)
                }
                Err(e) => {
                    eprintln!("Error parsing {}: {}", path.to_str().unwrap(), e);
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                eprintln!("Error reading: {}", e);
                Err(Box::new(e))
            }
        }
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
