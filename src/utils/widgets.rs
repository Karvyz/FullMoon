use iced::{
    Element, Font,
    font::Weight,
    widget::{Button, Image, image, image::Handle},
};
use libmoon::{message::OwnerType, persona::Persona};

use crate::{AppCommand, settings_page::SettingsPage};

pub fn text<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    settings: &'a SettingsPage,
) -> Element<'a, AppCommand> {
    iced::widget::text(content)
        .size(settings.font_size())
        .into()
}

pub fn bold_text<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    settings: &'a SettingsPage,
) -> Element<'a, AppCommand> {
    iced::widget::text(content)
        .size(settings.font_size())
        .font(Font {
            weight: Weight::Bold,
            ..Default::default()
        })
        .into()
}

pub fn button<'a>(content: &'a str, settings: &'a SettingsPage) -> Button<'a, AppCommand> {
    iced::widget::button(text(content, settings))
}

pub fn default_image(owner: &OwnerType) -> Image {
    match owner {
        libmoon::message::OwnerType::User => image("assets/user.png"),
        libmoon::message::OwnerType::Char(_) => image("assets/char.png"),
    }
}

pub fn persona_image(persona: &Persona) -> Image {
    match persona.raw_image() {
        Some((width, height, content)) => image(Handle::from_rgba(width, height, content)),
        None => image("assets/char.png"),
    }
}
