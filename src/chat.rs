use chrono::Local;
use iced::alignment::Horizontal;
use iced::font::Weight;
use iced::widget::keyed::Column;
use iced::widget::text::Shaping;
use iced::widget::{
    Scrollable, button, column, container, image, rich_text, row, scrollable, span, text,
};
use iced::{Border, Length::Fill, Theme};
use iced::{Font, Length};
use llm::chat::ChatMessage;

use crate::AppCommand;
use crate::message::{Message, MessageOwner};

#[derive(Debug, Clone)]
pub enum MessageCommand {
    Next,
    Previous,
}

#[derive(Default)]
pub struct Chat {
    childs: Vec<MessageNode>,
    selected: usize,
}

impl Chat {
    pub fn push(&mut self, message: Message) {
        match self.childs.is_empty() {
            true => self.childs.push(MessageNode::new(message)),
            false => self.childs[self.selected].push(message),
        }
    }

    pub fn get_current_chat(&self) -> Vec<Message> {
        let mut chat = vec![];
        if !self.childs.is_empty() {
            self.childs[self.selected].get_current_chat(&mut chat);
        }
        chat
    }

    pub fn get_chat_messages(&self) -> Vec<ChatMessage> {
        self.get_current_chat()
            .iter()
            .map(|msg| msg.to_chat_message())
            .collect()
    }

    pub fn get_chat_messages_until(&self, idx: usize) -> Vec<ChatMessage> {
        let mut chat = vec![];
        if !self.childs.is_empty() && idx > 0 {
            self.childs[self.selected].get_current_chat_until(&mut chat, idx - 1);
        }
        println!("Chat history len : {}", chat.len());
        chat.iter().map(|msg| msg.to_chat_message()).collect()
    }

    pub fn append_last_message(&mut self, text: &str) {
        match self.childs.is_empty() {
            true => eprintln!("Error: Trying to append to non existing message"),
            false => self.childs[self.selected].append_last_message(text),
        }
    }

    pub fn previous(&mut self, idx: usize) {
        match idx == 0 {
            true => {
                if self.selected > 0 {
                    self.selected -= 1
                }
            }
            false => self.childs[self.selected].previous(idx - 1),
        }
    }

    pub fn next(&mut self, idx: usize) -> bool {
        match idx == 0 {
            true => match self.selected < self.childs.len() - 1 {
                true => {
                    self.selected += 1;
                    false
                }
                false => {
                    self.selected += 1;
                    self.childs
                        .push(MessageNode::new(Message::empty(MessageOwner::Char)));
                    true
                }
            },
            false => self.childs[self.selected].next(idx - 1),
        }
    }

    pub fn view(&self) -> Scrollable<'_, AppCommand> {
        scrollable(self.create_column_view())
            .anchor_bottom()
            .height(Fill)
            .width(Fill)
            .spacing(10)
    }

    fn create_column_view(&self) -> Column<'_, usize, AppCommand> {
        let mut keyed_column = Column::new().spacing(10);
        let mut nb_childs = self.childs.len();
        let mut selected = self.selected;
        if nb_childs == 0 {
            return keyed_column;
        }
        let mut current_node = &self.childs[selected];
        let mut idx = 0;
        loop {
            keyed_column = keyed_column.push(
                idx,
                container(
                    row![
                        container(
                            image(match current_node.message.owner {
                                MessageOwner::User => "assets/user.png",
                                MessageOwner::Char => "assets/char.png",
                            })
                            .width(100)
                            .height(100)
                        ),
                        column![
                            rich_text![
                                span(match current_node.message.owner {
                                    MessageOwner::User => "User",
                                    MessageOwner::Char => "Char",
                                })
                                .font(Font {
                                    weight: Weight::Bold,
                                    ..Font::default()
                                }),
                                "  ",
                                span(Local::now().format("%B %d, %Y %H:%M").to_string())
                            ],
                            text(current_node.message.text.clone())
                                .width(Fill)
                                .shaping(Shaping::Advanced)
                        ]
                        .spacing(4),
                        column![
                            text(format!("{}/{}", selected + 1, nb_childs))
                                .align_x(Horizontal::Center),
                            button(">")
                                .on_press(AppCommand::MessageCommand(idx, MessageCommand::Next)),
                            button("<").on_press(AppCommand::MessageCommand(
                                idx,
                                MessageCommand::Previous
                            ))
                        ]
                        .spacing(2)
                        .width(Length::Shrink)
                    ]
                    .padding(10)
                    .spacing(10),
                )
                .style(Self::message_style),
            );
            idx += 1;
            if current_node.childs.is_empty() {
                return keyed_column;
            }
            nb_childs = current_node.childs.len();
            selected = current_node.selected;
            current_node = &current_node.childs[current_node.selected];
        }
    }

    fn message_style(theme: &Theme) -> iced::widget::container::Style {
        let palette = theme.extended_palette();
        container::rounded_box(theme)
            .background(palette.background.weak.color)
            .border(Border::default().rounded(12))
    }
}

struct MessageNode {
    message: Message,
    childs: Vec<MessageNode>,
    selected: usize,
}

impl MessageNode {
    fn new(message: Message) -> Self {
        MessageNode {
            message,
            childs: vec![],
            selected: 0,
        }
    }

    fn push(&mut self, message: Message) {
        match self.childs.is_empty() {
            true => self.childs.push(MessageNode::new(message)),
            false => self.childs[self.selected].push(message),
        }
    }

    fn get_current_chat(&self, chat: &mut Vec<Message>) {
        chat.push(self.message.clone());
        if !self.childs.is_empty() {
            self.childs[self.selected].get_current_chat(chat);
        }
    }

    fn get_current_chat_until(&self, chat: &mut Vec<Message>, idx: usize) {
        chat.push(self.message.clone());
        if !self.childs.is_empty() && idx > 0 {
            self.childs[self.selected].get_current_chat_until(chat, idx - 1);
        }
    }

    fn append_last_message(&mut self, text: &str) {
        match self.childs.is_empty() {
            true => self.message.text.push_str(text),
            false => self.childs[self.selected].append_last_message(text),
        }
    }

    fn previous(&mut self, idx: usize) {
        match idx == 0 {
            true => {
                if self.selected > 0 {
                    self.selected -= 1
                }
            }
            false => self.childs[self.selected].previous(idx - 1),
        }
    }

    fn next(&mut self, idx: usize) -> bool {
        match idx == 0 {
            true => match self.selected < self.childs.len() - 1 {
                true => {
                    self.selected += 1;
                    false
                }
                false => {
                    self.selected += 1;
                    self.childs
                        .push(MessageNode::new(Message::empty(MessageOwner::Char)));
                    true
                }
            },
            false => self.childs[self.selected].next(idx - 1),
        }
    }
}
