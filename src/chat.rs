use iced::{
    Border,
    Length::Fill,
    Theme,
    widget::{Scrollable, Text, container, image, keyed_column, row, scrollable},
};

use crate::IcedMessage;

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
                                    Text::new(message.text.clone()).width(Fill)
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
    }

    fn message_style(theme: &Theme) -> iced::widget::container::Style {
        let palette = theme.extended_palette();
        container::rounded_box(theme)
            .background(palette.background.weak.color)
            .border(Border::default().rounded(12))
    }
}

#[derive(Debug, Clone)]
pub enum MessageOwner {
    User,
    Char,
}

#[derive(Clone)]
pub struct Message {
    pub owner: MessageOwner,
    pub text: String,
}

impl Message {
    pub fn new(owner: MessageOwner, text: String) -> Self {
        let text = text.trim().to_string();
        println!("{:?}: {:?}", owner, text);
        Message { owner, text }
    }
}
