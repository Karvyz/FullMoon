use iced::{Element, Font, font::Weight, widget::Button};

use crate::{AppCommand, settings::Settings};

pub fn text<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    settings: &'a Settings,
) -> Element<'a, AppCommand> {
    iced::widget::text(content)
        .size(settings.font_size())
        .into()
}

pub fn bold_text<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    settings: &'a Settings,
) -> Element<'a, AppCommand> {
    iced::widget::text(content)
        .size(settings.font_size())
        .font(Font {
            weight: Weight::Bold,
            ..Default::default()
        })
        .into()
}

pub fn button<'a>(content: &'a str, settings: &'a Settings) -> Button<'a, AppCommand> {
    iced::widget::button(text(content, settings))
}
