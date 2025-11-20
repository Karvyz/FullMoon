use iced::{
    Alignment, Element, Task,
    widget::{
        TextEditor, row,
        text_editor::{Action, Content},
    },
};
use llm::chat::ChatMessage;

use crate::{
    AppCommand,
    chat_page::chat::Chat,
    message::Message,
    persona::{
        Persona,
        loader::{PersonaLoader, Subdir},
    },
    settings::Settings,
    utils::widgets::{bold_text, button},
};

mod chat;

#[derive(Debug, Clone)]
pub enum ChatCommand {
    InputChange(Action),
    InputSubmit,
    GenerateNextMessage,
    StreamOk(String),
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
    char: Persona,
    user: Persona,
}

impl Default for ChatPage {
    fn default() -> Self {
        ChatPage {
            input_message: Content::new(),
            chat: Chat::default(),
            char: Persona::default_char(),
            user: Persona::default_user(),
        }
    }
}

impl ChatPage {
    pub fn new(char: Persona, user: Persona) -> Self {
        ChatPage {
            input_message: Content::new(),
            chat: Chat::with_messages(&char, &user),
            char,
            user,
        }
    }

    pub fn try_load() -> Self {
        let char = PersonaLoader::load_most_recent_from_cache(Subdir::Chars);
        let user = PersonaLoader::load_most_recent_from_cache(Subdir::Users);
        ChatPage::new(char, user)
    }

    pub fn set_char(&mut self, char: Persona) {
        self.char = char;
        self.new_chat();
    }

    pub fn new_chat(&mut self) {
        self.chat = Chat::with_messages(&self.char, &self.user);
    }

    pub fn view<'a>(&'a self, settings: &'a Settings) -> Element<'a, AppCommand> {
        iced::widget::column![
            bold_text(
                format!("{}'s chat with {}", self.user.name(), self.char.name()),
                settings
            ),
            self.chat.view(settings),
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

    pub fn update(&mut self, chat_command: ChatCommand, settings: &Settings) -> Task<AppCommand> {
        match chat_command {
            ChatCommand::InputChange(action) => self.input_message.perform(action),
            ChatCommand::InputSubmit => {
                let text = self.input_message.text().trim().to_string();
                if !text.is_empty() {
                    self.chat.push(Message::from_user(self.user.clone(), text));
                    self.input_message = Content::new();
                }
                return Task::done(ChatCommand::GenerateNextMessage.into());
            }
            ChatCommand::GenerateNextMessage => {
                let chat_history = self.chat.get_chat_messages();
                self.chat.push(Message::empty_from_char(self.char.clone()));
                return self.get_response(settings, chat_history);
            }
            ChatCommand::StreamOk(text) => self.chat.append_last_message(text.as_str()),
            ChatCommand::MessageCommand(message_command) => match message_command {
                MessageCommand::Next(idx) => {
                    if self.chat.next(idx, self.char.clone()) {
                        return self.get_response(settings, self.chat.get_chat_messages_until(idx));
                    }
                }
                MessageCommand::Previous(idx) => self.chat.previous(idx),
                MessageCommand::ToggleEdit(idx) => {
                    if self.chat.toggle_edit(idx) {
                        return Task::done(ChatCommand::GenerateNextMessage.into());
                    }
                }
                MessageCommand::AbortEdit(idx) => self.chat.abort_edit(idx),
                MessageCommand::EditAction(idx, action) => self.chat.perform_action(idx, action),
                MessageCommand::Delete(idx) => self.chat.delete(idx),
            },
        }
        Task::none()
    }

    fn get_response(&self, settings: &Settings, messages: Vec<ChatMessage>) -> Task<AppCommand> {
        let llm = settings.llm(&self.char, &self.user);
        Task::perform(async move { llm.chat_stream(&messages).await }, |res| res).and_then(|res| {
            Task::run(res, |chunk| match chunk {
                Ok(text) => ChatCommand::StreamOk(text).into(),
                Err(e) => AppCommand::Error(e.to_string()),
            })
        })
    }
}
