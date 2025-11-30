use crate::{
    AppCommand,
    formater::Formater,
    settings::Settings,
    utils::widgets::{bold_text, button, text},
};
use chrono::Local;
use iced::{
    Alignment, Border, Element, Font,
    Length::{self, Fill},
    Task, Theme,
    font::Weight,
    widget::{
        TextEditor, column, container, keyed, rich_text, row, scrollable, span,
        text_editor::{Action, Content},
    },
};
use iced_modern_theme::colors::colors;
use libmoon::{chat::Chat, persona::Persona};

#[derive(Debug, Clone)]
pub enum ChatCommand {
    InputChange(Action),
    InputSubmit,
    MessageCommand(MessageCommand),
}

impl From<ChatCommand> for crate::AppCommand {
    fn from(chat_command: ChatCommand) -> Self {
        crate::AppCommand::ChatCommand(chat_command)
    }
}

#[derive(Debug, Clone)]
pub enum MessageCommand {
    Next(usize),
    Previous(usize),
    ToggleEdit(usize),
    AbortEdit(usize),
    EditAction(usize, Action),
    Delete(usize),
}

impl From<MessageCommand> for crate::AppCommand {
    fn from(message_command: MessageCommand) -> Self {
        crate::AppCommand::ChatCommand(ChatCommand::MessageCommand(message_command))
    }
}

pub struct ChatPage {
    chat: Chat,
    input_message: Content,
}

impl Default for ChatPage {
    fn default() -> Self {
        ChatPage {
            input_message: Content::new(),
            chat: Chat::load(),
        }
    }
}

impl ChatPage {
    pub fn new(char: Persona, user: Persona, settings: libmoon::settings::Settings) -> Self {
        ChatPage {
            input_message: Content::new(),
            chat: Chat::with_personas(user, char, settings),
        }
    }

    pub fn try_load() -> Self {
        ChatPage {
            input_message: Content::new(),
            chat: Chat::load(),
        }
    }

    pub fn set_char(&mut self, char: Persona) {
        let user = self.chat.user();
        let settings = self.chat.settings().clone();
        self.chat = Chat::with_personas(user, char, settings);
    }

    pub fn view<'a>(&'a self, settings: &'a Settings) -> Element<'a, AppCommand> {
        iced::widget::column![
            bold_text(self.chat.title(), settings),
            self.chat_view(settings),
            row![
                TextEditor::new(&self.input_message)
                    .size(settings.font_size())
                    .key_binding(crate::utils::binds::from_key_press)
                    .on_action(|a| ChatCommand::InputChange(a).into()),
                button("Submit", settings).on_press(ChatCommand::InputSubmit.into())
            ]
            .spacing(10),
        ]
        .align_x(Alignment::Center)
        .padding(20)
        .spacing(10)
        .into()
    }

    pub fn chat_view<'a>(&'a self, settings: &'a Settings) -> Element<'a, AppCommand> {
        scrollable(self.create_column_view(settings))
            .anchor_bottom()
            .height(Fill)
            .width(Fill)
            .spacing(10)
            .into()
    }

    fn create_column_view<'a>(
        &'a self,
        settings: &'a Settings,
    ) -> keyed::Column<'a, usize, AppCommand> {
        let mut keyed_column = keyed::Column::new().spacing(10);
        let messages = self.chat.get_history();
        let structure = self.chat.get_history_structure();

        for (idx, message) in messages.into_iter().enumerate() {
            keyed_column = keyed_column.push(
                idx,
                container(
                    row![
                        // image(current_node.message.get_avatar_uri())
                        // current_node.message.owner.image().width(Fill),
                        column![
                            row![
                                rich_text![
                                    span(self.chat.owner_name(&message))
                                        .font(Font {
                                            weight: Weight::Bold,
                                            ..Font::default()
                                        })
                                        .size(settings.font_size()),
                                    "  ",
                                    span(Local::now().format("%B %d, %Y %H:%M").to_string())
                                        .size(settings.font_size())
                                ]
                                .width(Fill),
                                text(
                                    format!("{}/{}", structure[idx].0, structure[idx].1),
                                    settings
                                ),
                                button("<", settings)
                                    .on_press(MessageCommand::Previous(idx).into()),
                                button(">", settings).on_press(MessageCommand::Next(idx).into()),
                                button("E", settings)
                                    .on_press(MessageCommand::ToggleEdit(idx).into()),
                                button("A", settings)
                                    .on_press(MessageCommand::AbortEdit(idx).into()),
                                button("D", settings).on_press(MessageCommand::Delete(idx).into())
                            ]
                            .align_y(Alignment::Center)
                            .spacing(2),
                            Element::from(Formater::rich_text(message.text.clone(), settings)),
                            // if let Some(edit) = &current_node.message.editing {
                            //     let idx2 = idx;
                            //     Element::from(
                            //         TextEditor::new(edit).size(settings.font_size()).on_action(
                            //             move |a| MessageCommand::EditAction(idx2, a).into(),
                            //         ),
                            //     )
                            // } else {
                            //     Element::from(Formater::rich_text(&message.text, settings))
                            // },
                        ]
                        .spacing(4)
                        .width(Length::FillPortion(6)),
                    ]
                    .padding(10)
                    .spacing(10),
                )
                .style(Self::message_style),
            );
        }
        keyed_column
    }

    fn message_style(theme: &Theme) -> iced::widget::container::Style {
        container::rounded_box(theme)
            .background(colors::fill::SECONDARY_DARK)
            .border(Border::default().rounded(12))
    }

    pub fn update(&mut self, chat_command: ChatCommand, settings: &Settings) -> Task<AppCommand> {
        match chat_command {
            ChatCommand::InputChange(action) => self.input_message.perform(action),
            ChatCommand::InputSubmit => {
                let text = self.input_message.text().trim().to_string();
                self.input_message = Content::new();
                if !text.is_empty() {
                    self.chat.add_user_message(text);
                }
            }
            ChatCommand::MessageCommand(message_command) => match message_command {
                MessageCommand::Next(idx) => self.chat.next(idx),
                MessageCommand::Previous(idx) => self.chat.previous(idx),
                MessageCommand::ToggleEdit(_) => todo!(),
                MessageCommand::AbortEdit(_) => todo!(),
                MessageCommand::EditAction(_, _) => todo!(),
                MessageCommand::Delete(_) => todo!(),
            },
        }
        Task::none()
    }
}
