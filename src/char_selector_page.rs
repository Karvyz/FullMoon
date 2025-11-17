use iced::{
    Border, Element, Font,
    Length::Fill,
    Theme,
    font::Weight,
    widget::{button, column, container, image, keyed, row, scrollable, text},
};
use iced_modern_theme::colors::colors;

use crate::{
    AppCommand,
    persona::{
        Persona,
        loader::{PersonaLoader, Subdir},
    },
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

    pub fn view(&self) -> Element<'_, AppCommand> {
        let mut keyed_column = keyed::Column::new().padding(10).spacing(10);
        for (idx, char) in self.chars.iter().enumerate() {
            keyed_column = keyed_column.push(
                idx,
                container(
                    row![
                        image(char.avatar_uri().unwrap_or("assets/char.png".to_string()))
                            .filter_method(image::FilterMethod::Linear)
                            .width(200)
                            .height(200),
                        column![
                            text(char.name()).font(Font {
                                weight: Weight::Bold,
                                ..Default::default()
                            }),
                            button("Edit"),
                            button("Select").on_press(AppCommand::SelectedChar(idx))
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
