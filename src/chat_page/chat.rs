use std::sync::Arc;

use chrono::Local;
use iced::{
    Border, Element, Font,
    Length::{self, Fill, Shrink},
    Theme,
    alignment::Horizontal,
    font::Weight,
    widget::{
        TextEditor, button, column, container, image,
        keyed::Column,
        rich_text, row, scrollable, span, text,
        text_editor::{Action, Content},
    },
};
use iced_modern_theme::colors::colors;
use llm::chat::ChatMessage;

use crate::{
    AppCommand, chat_page::MessageCommand, formater::Formater, message::Message, persona::Persona,
};

#[derive(Default)]
pub struct Chat {
    childs: Vec<MessageNode>,
    selected: usize,
}

impl Chat {
    pub fn with_messages(char: &Arc<Persona>, user: &Arc<Persona>) -> Self {
        Chat {
            childs: match char.greetings(Some(&user.name())) {
                Some(messages) => messages
                    .iter()
                    .map(|m| MessageNode::new(Message::from_char(char.clone(), m.clone())))
                    .collect(),
                None => vec![],
            },
            selected: 0,
        }
    }

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

    pub fn next(&mut self, idx: usize, char: Arc<Persona>) -> bool {
        match idx == 0 {
            true => match self.selected < self.childs.len() - 1 {
                true => {
                    self.selected += 1;
                    false
                }
                false => {
                    self.selected += 1;
                    self.childs
                        .push(MessageNode::new(Message::empty_from_char(char)));
                    true
                }
            },
            false => self.childs[self.selected].next(idx - 1, char),
        }
    }

    pub fn toggle_edit(&mut self, idx: usize) {
        match idx == 0 {
            true => match &self.childs[self.selected].message.editing {
                Some(content) => {
                    let mut new_child = self.childs[self.selected].message.clone();
                    new_child.text = content.text();
                    self.childs[self.selected].message.editing = None;
                    self.childs.push(MessageNode::new(new_child));
                    self.selected = self.childs.len() - 1;
                }
                None => {
                    self.childs[self.selected].message.editing =
                        Some(Content::with_text(&self.childs[self.selected].message.text))
                }
            },
            false => self.childs[self.selected].toggle_edit(idx - 1),
        }
    }

    pub fn abort_edit(&mut self, idx: usize) {
        match idx == 0 {
            true => self.childs[self.selected].message.editing = None,
            false => self.childs[self.selected].abort_edit(idx - 1),
        }
    }

    pub fn perform_action(&mut self, idx: usize, action: Action) {
        match idx == 0 {
            true => match &mut self.childs[self.selected].message.editing {
                Some(content) => content.perform(action),
                None => eprintln!("Content not found"),
            },
            false => self.childs[self.selected].perform_action(idx - 1, action),
        }
    }

    pub fn view(&self) -> Element<'_, AppCommand> {
        scrollable(self.create_column_view())
            .anchor_bottom()
            .height(Fill)
            .width(Fill)
            .spacing(10)
            .into()
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
            keyed_column =
                keyed_column.push(
                    idx,
                    container(
                        row![
                            image(current_node.message.get_avatar_uri())
                                .filter_method(image::FilterMethod::Linear)
                                .width(Fill),
                            column![
                                rich_text![
                                    span(current_node.message.owner.name()).font(Font {
                                        weight: Weight::Bold,
                                        ..Font::default()
                                    }),
                                    "  ",
                                    span(Local::now().format("%B %d, %Y %H:%M").to_string())
                                ]
                                .width(Shrink),
                                if let Some(edit) = &current_node.message.editing {
                                    let idx2 = idx;
                                    Element::from(TextEditor::new(edit).on_action(move |a| {
                                        MessageCommand::EditAction(idx2, a).into()
                                    }))
                                } else {
                                    Element::from(Formater::rich_text(&current_node.message.text))
                                },
                            ]
                            .spacing(4)
                            .width(Length::FillPortion(6)),
                            column![
                                text(format!("{}/{}", selected + 1, nb_childs)),
                                button(text(">")).on_press(MessageCommand::Next(idx).into()),
                                button("<").on_press(MessageCommand::Previous(idx).into()),
                                button("E").on_press(MessageCommand::ToggleEdit(idx).into()),
                                button("A").on_press(MessageCommand::AbortEdit(idx).into())
                            ]
                            .spacing(2)
                            .align_x(Horizontal::Right)
                            .width(Length::Fill)
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
        container::rounded_box(theme)
            .background(colors::fill::SECONDARY_DARK)
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

    fn next(&mut self, idx: usize, char: Arc<Persona>) -> bool {
        match idx == 0 {
            true => match self.selected < self.childs.len() - 1 {
                true => {
                    self.selected += 1;
                    false
                }
                false => {
                    self.selected += 1;
                    self.childs
                        .push(MessageNode::new(Message::empty_from_char(char)));
                    true
                }
            },
            false => self.childs[self.selected].next(idx - 1, char),
        }
    }

    pub fn toggle_edit(&mut self, idx: usize) {
        match idx == 0 {
            true => {
                if let Some(content) = &self.childs[self.selected].message.editing {
                    let mut new_child = self.childs[self.selected].message.clone();
                    new_child.text = content.text();
                    self.childs[self.selected].message.editing = None;
                    self.childs.push(MessageNode::new(new_child));
                    self.selected = self.childs.len() - 1;
                }
            }
            false => self.childs[self.selected].toggle_edit(idx - 1),
        }
    }

    pub fn abort_edit(&mut self, idx: usize) {
        match idx == 0 {
            true => self.childs[self.selected].message.editing = None,
            false => self.childs[self.selected].abort_edit(idx - 1),
        }
    }

    pub fn perform_action(&mut self, idx: usize, action: Action) {
        match idx == 0 {
            true => match &mut self.childs[self.selected].message.editing {
                Some(content) => content.perform(action),
                None => eprintln!("Content not found"),
            },
            false => self.childs[self.selected].perform_action(idx - 1, action),
        }
    }
}
