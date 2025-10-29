use chrono::Local;
use iced::alignment::Horizontal;
use iced::font::Weight;
use iced::widget::text::Shaping;
use iced::widget::{
    Scrollable, button, column, container, image, keyed_column, rich_text, row, scrollable, span,
    text,
};
use iced::{Border, Length::Fill, Theme};
use iced::{Font, Length};
use llm::chat::ChatMessage;

use crate::IcedMessage;
use crate::message::{Message, MessageOwner};

#[derive(Default)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> Vec<Message> {
        self.messages.clone()
    }

    pub fn get_chat_messages(&self) -> Vec<ChatMessage> {
        self.get_messages()
            .iter()
            .map(|msg| msg.to_chat_message())
            .collect()
    }

    pub fn append_last_message(&mut self, text: String) {
        let idx = self.messages.len() - 1;
        self.messages[idx].text.push_str(text.as_str());
    }

    pub fn view(&self) -> Scrollable<'_, IcedMessage> {
        scrollable(
            keyed_column(
                self.get_messages()
                    .iter()
                    .enumerate()
                    .map(|(idx, message)| {
                        (
                            idx,
                            container(
                                row![
                                    container(
                                        image(match message.owner {
                                            MessageOwner::User => "assets/user.png",
                                            MessageOwner::Char => "assets/char.png",
                                        })
                                        .width(100)
                                        .height(100)
                                    ),
                                    column![
                                        rich_text![
                                            span(match message.owner {
                                                MessageOwner::User => "User",
                                                MessageOwner::Char => "Char",
                                            })
                                            .font(
                                                Font {
                                                    weight: Weight::Bold,
                                                    ..Font::default()
                                                }
                                            ),
                                            "  ",
                                            span(
                                                Local::now().format("%B %d, %Y %H:%M").to_string()
                                            )
                                        ],
                                        text(message.text.clone())
                                            .width(Fill)
                                            .shaping(Shaping::Advanced)
                                    ]
                                    .spacing(4),
                                    column![
                                        text("1/1").align_x(Horizontal::Center),
                                        button(">"),
                                        button("<")
                                    ]
                                    .spacing(2)
                                    .width(Length::Shrink)
                                ]
                                .padding(10)
                                .spacing(10),
                            )
                            .style(Self::message_style)
                            .into(),
                        )
                    }),
            )
            .spacing(10),
        )
        .anchor_bottom()
        .height(Fill)
        .width(Fill)
        .spacing(10)
    }

    fn message_style(theme: &Theme) -> iced::widget::container::Style {
        let palette = theme.extended_palette();
        container::rounded_box(theme)
            .background(palette.background.weak.color)
            .border(Border::default().rounded(12))
    }
}
