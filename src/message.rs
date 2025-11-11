use std::sync::Arc;

use crate::{AppCommand, persona::Persona};
use iced::{
    Color, Element, Font, Length,
    font::Style,
    widget::{span, text::Rich},
};
use iced_modern_theme::colors::colors;
use llm::chat::ChatMessage;

#[derive(Clone)]
pub enum OwnerType {
    User,
    Char,
}

#[derive(Clone)]
pub struct Message {
    pub owner: Arc<Persona>,
    pub owner_type: OwnerType,
    pub text: String,
}

impl Message {
    pub fn from_user(user: Arc<Persona>, text: String) -> Self {
        Message {
            owner: user,
            owner_type: OwnerType::User,
            text: text.trim().to_string(),
        }
    }

    pub fn from_char(char: Arc<Persona>, text: String) -> Self {
        Message {
            owner: char,
            owner_type: OwnerType::Char,
            text: text.trim().to_string(),
        }
    }

    pub fn empty_from_char(char: Arc<Persona>) -> Self {
        Self::from_char(char, String::new())
    }

    pub fn to_chat_message(&self) -> ChatMessage {
        match self.owner_type {
            OwnerType::User => ChatMessage::user().content(&self.text).build(),
            OwnerType::Char => ChatMessage::assistant().content(&self.text).build(),
        }
    }

    pub fn get_avatar_uri(&self) -> String {
        match self.owner.avatar_uri() {
            Some(uri) => uri,
            None => match self.owner_type {
                OwnerType::User => "assets/user.png".to_string(),
                OwnerType::Char => "assets/char.png".to_string(),
            },
        }
    }

    pub fn rich_text(&self) -> Element<'_, AppCommand> {
        let mut spans = vec![];
        let mut current_type = StringType::Normal;
        let mut current_string = String::new();
        let mut push_char_anyway = false;
        let mut push_before = false;
        for char in self.text.chars() {
            let nt = match char {
                '*' => match current_type {
                    StringType::Normal => Some(StringType::Strong),
                    StringType::Strong => Some(StringType::Normal),
                    StringType::Quote => Some(StringType::StrongQuote),
                    StringType::StrongQuote => Some(StringType::Quote),
                },
                '"' => {
                    push_char_anyway = true;
                    match current_type {
                        StringType::Normal => Some(StringType::Quote),
                        StringType::Quote => {
                            push_before = true;
                            Some(StringType::Normal)
                        }
                        StringType::Strong => Some(StringType::StrongQuote),
                        StringType::StrongQuote => {
                            push_before = true;
                            Some(StringType::Strong)
                        }
                    }
                }
                _ => None,
            };
            match nt {
                Some(nt) => {
                    if push_char_anyway && push_before {
                        current_string.push(char);
                    }
                    spans.push(
                        span(current_string)
                            .color(current_type.clone())
                            .font(current_type),
                    );
                    current_type = nt;
                    current_string = String::new();
                    if push_char_anyway && !push_before {
                        current_string.push(char);
                    }
                }
                None => current_string.push(char),
            }
            push_char_anyway = false;
            push_before = false;
        }
        spans.push(
            span(current_string)
                .color(current_type.clone())
                .font(current_type),
        );
        Rich::with_spans(spans).width(Length::Shrink).into()
    }
}

#[derive(Clone)]
enum StringType {
    Normal,
    Strong,
    Quote,
    StrongQuote,
}

impl From<StringType> for Font {
    fn from(value: StringType) -> Self {
        match value {
            StringType::Strong => Font {
                style: Style::Italic,
                ..Default::default()
            },
            StringType::StrongQuote => Font {
                style: Style::Italic,
                ..Font::default()
            },
            _ => Font::default(),
        }
    }
}

impl From<StringType> for Color {
    fn from(value: StringType) -> Self {
        match value {
            StringType::Normal => colors::text::PRIMARY_DARK,
            StringType::Strong => colors::text::SECONDARY_DARK,
            StringType::Quote => colors::system::ORANGE,
            StringType::StrongQuote => colors::system::ORANGE,
        }
    }
}
