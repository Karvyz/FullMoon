use iced::{
    Color, Element, Font, Length,
    font::Style,
    widget::{span, text::Rich},
};
use iced_modern_theme::colors::colors;
use regex::Regex;

use crate::AppCommand;

pub struct Formater {}

impl Formater {
    fn clean(text: &str) -> String {
        // Remove markdown images
        let image_re = Regex::new(r"!\[[^\]]*\]\([^)]*\)").unwrap();
        let no_images = image_re.replace_all(text, "");

        // Normalize all line endings to \n first
        let no_backr = no_images.replace("\r\n", "\n");

        // Replace 3+ consecutive newlines with exactly 2
        let re_newlines = Regex::new(r"\n{3,}").unwrap();
        let few_linebreaks = re_newlines.replace_all(&no_backr, "\n\n").to_string();

        // Trim whitespace from start and end
        few_linebreaks.trim().to_string()
    }

    pub fn rich_text(text: &str) -> Element<'_, AppCommand> {
        let mut spans = vec![];
        let mut current_type = StringType::Normal;
        let mut current_string = String::new();
        let mut push_char_anyway = false;
        let mut push_before = false;
        for char in Self::clean(text).chars() {
            let nt = match char {
                '*' => match current_type {
                    StringType::Normal => Some(StringType::Strong),
                    StringType::Strong => Some(StringType::Normal),
                    StringType::Quote => Some(StringType::StrongQuote),
                    StringType::StrongQuote => Some(StringType::Quote),
                },
                '"' | '“' | '”' => {
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

