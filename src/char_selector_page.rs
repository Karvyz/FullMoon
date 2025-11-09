use iced::{
    Element,
    Length::Fill,
    widget::{button, column, container, keyed, row, scrollable, text},
};

use crate::{
    AppCommand,
    persona::{Persona, PersonaLoader},
};

pub struct CharSelectorPage {
    chars: Vec<Persona>,
}

impl CharSelectorPage {
    pub fn new() -> Self {
        Self {
            chars: PersonaLoader::load_from_cache("personas"),
        }
    }

    pub fn get(&self, idx: usize) -> Persona {
        self.chars[idx].clone()
    }

    pub fn view(&self) -> Element<'_, AppCommand> {
        let mut keyed_column = keyed::Column::new();
        for (idx, char) in self.chars.iter().enumerate() {
            keyed_column = keyed_column.push(
                idx,
                container(row![
                    column![text(char.name()), text(char.description())],
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
