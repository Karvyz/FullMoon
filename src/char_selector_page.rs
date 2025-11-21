use iced::{
    Border, Element,
    Length::Fill,
    Theme,
    widget::{column, container, keyed, row, scrollable},
};
use iced_modern_theme::colors::colors;

use crate::{
    AppCommand,
    persona::{
        Persona,
        loader::{PersonaLoader, Subdir},
    },
    settings::Settings,
    utils::widgets::{bold_text, button},
};

pub struct CharSelectorPage {
    chars: Vec<Persona>,
}

impl CharSelectorPage {
    pub fn new() -> Self {
        Self {
            chars: PersonaLoader::load_from_cache(Subdir::Chars),
        }
    }

    pub fn get(&self, idx: usize) -> Persona {
        self.chars[idx].clone()
    }

    pub fn view<'a>(&'a self, settings: &'a Settings) -> Element<'a, AppCommand> {
        let mut keyed_column = keyed::Column::new().padding(10).spacing(10);
        for (idx, char) in self.chars.iter().enumerate() {
            keyed_column = keyed_column.push(
                idx,
                container(
                    row![
                        char.image().height(200),
                        column![
                            bold_text(char.name(), settings),
                            button("Edit", settings),
                            button("Select", settings).on_press(AppCommand::SelectedChar(idx))
                        ]
                        .width(Fill)
                        .spacing(10)
                    ]
                    .width(Fill)
                    .spacing(10)
                    .padding(10),
                )
                .style(Self::charbox_style),
            )
        }
        scrollable(keyed_column)
            .anchor_bottom()
            .height(Fill)
            .width(Fill)
            .spacing(10)
            .into()
    }

    fn charbox_style(theme: &Theme) -> iced::widget::container::Style {
        container::rounded_box(theme)
            .background(colors::fill::SECONDARY_DARK)
            .border(Border::default().rounded(12))
    }
}
