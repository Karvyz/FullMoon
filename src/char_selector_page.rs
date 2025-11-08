use std::fs;

use iced::{
    Element,
    Length::Fill,
    widget::{button, column, container, keyed, row, scrollable, text},
};

use crate::{
    AppCommand,
    persona::{Persona, char::Char},
};

pub struct CharSelectorPage {
    chars: Vec<Char>,
}

impl CharSelectorPage {
    pub fn new() -> Self {
        let mut chars = vec![];

        println!("Loading config started");
        let pathbuf = dirs::cache_dir()
            .map(|mut path| {
                path.push("fullmoon");
                path.push("personas");
                path
            })
            .unwrap();

        match fs::read_dir(&pathbuf) {
            Err(_) => {
                println!("Cache not found. Writing default");
                let char = Char::default();
                if char.save(pathbuf).is_err() {
                    eprintln!("Writing default failed");
                }
                chars.push(char);
            }
            Ok(entries) => {
                for entry in entries {
                    let entry = entry.unwrap();
                    let path = entry.path();

                    // Check if it's a file (not a directory)
                    if path.is_file() {
                        match fs::read_to_string(&path) {
                            Ok(data) => match Char::load_from_json(&data) {
                                Ok(char) => {
                                    println!("Loaded {}", char.get_name());
                                    chars.push(char);
                                }
                                Err(e) => {
                                    eprintln!("Error parsing {}: {}", path.to_str().unwrap(), e)
                                }
                            },
                            Err(e) => eprintln!("Error reading: {}", e),
                        }
                    }
                }
            }
        }
        Self { chars }
    }

    pub fn get(&self, idx: usize) -> Char {
        self.chars[idx].clone()
    }

    pub fn view(&self) -> Element<'_, AppCommand> {
        let mut keyed_column = keyed::Column::new();
        for (idx, char) in self.chars.iter().enumerate() {
            keyed_column = keyed_column.push(
                idx,
                container(row![
                    column![text(char.get_name()), text(char.get_description())],
                    button("Select").on_press(AppCommand::SelectedChar(idx))
                ]),
            )
        }
        scrollable(keyed_column)
            .anchor_bottom()
            .height(Fill)
            .width(Fill)
            .spacing(10)
            .into()
    }
}
