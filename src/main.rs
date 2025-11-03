mod chat;
mod message;
mod persona;
mod settings;

use std::sync::Arc;
use std::time::Duration;

use chat::Chat;
use iced::widget::{Stack, button, column, container, text, text_input};
use iced::{Border, Center, Element, Length, Task, Theme};
use llm::chat::ChatMessage;
use tokio::time::sleep;

use crate::chat::MessageCommand;
use crate::message::Message;
use crate::persona::Persona;
use crate::persona::char::Char;
use crate::persona::user::User;
use crate::settings::Settings;

pub fn main() -> iced::Result {
    iced::application("FullMoon", App::update, App::view)
        .theme(App::theme)
        .run_with(|| (App::new(), iced::Task::none()))
}

struct App {
    input_message: String,
    chat: Chat,
    settings: Settings,
    char: Arc<dyn Persona>,
    user: Arc<dyn Persona>,
    error: Option<String>,
}

#[derive(Debug, Clone)]
enum AppCommand {
    InputChange(String),
    CreateMessage,
    StreamOk(String),
    MessageCommand(MessageCommand),
    Error(String),
    DismissError,
}

impl App {
    fn new() -> Self {
        App {
            input_message: String::new(),
            chat: Chat::default(),
            settings: Settings::load(),
            char: Arc::new(Char::default()),
            user: Arc::new(User::default()),
            error: None,
        }
    }

    fn update(&mut self, message: AppCommand) -> Task<AppCommand> {
        match message {
            AppCommand::InputChange(input) => self.input_message = input,
            AppCommand::CreateMessage => {
                self.create_message();
                let chat_history = self.chat.get_chat_messages();
                self.chat.push(Message::empty(self.char.clone()));
                return self.get_response(chat_history);
            }
            AppCommand::StreamOk(text) => self.chat.append_last_message(text.as_str()),
            AppCommand::MessageCommand(message_command) => match message_command {
                MessageCommand::Next(idx) => {
                    if self.chat.next(idx, self.char.clone()) {
                        return self.get_response(self.chat.get_chat_messages_until(idx));
                    }
                }
                MessageCommand::Previous(idx) => self.chat.previous(idx),
            },
            AppCommand::Error(e) => {
                self.error = Some(e);
                return Task::perform(sleep(Duration::from_secs(3)), |_| AppCommand::DismissError);
            }
            AppCommand::DismissError => self.error = None,
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, AppCommand> {
        let mut stack = Stack::new();
        stack = stack.push(
            column![
                self.chat.view(),
                text_input("What needs to be done?", &self.input_message)
                    .id("user-input")
                    .on_input(AppCommand::InputChange)
                    .on_submit(AppCommand::CreateMessage),
                button("Error").on_press(AppCommand::Error("Button".to_string()))
            ]
            .padding(20)
            .align_x(Center)
            .spacing(10),
        );
        if let Some(e) = &self.error {
            stack = stack.push(
                container(container(text(e)).padding(20).style(Self::error_style))
                    .center_x(Length::Fill)
                    .padding(20),
            );
        }
        stack.into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    fn error_style(theme: &Theme) -> iced::widget::container::Style {
        let palette = theme.extended_palette();
        container::rounded_box(theme)
            .background(palette.background.weak.color)
            .border(
                Border::default()
                    .rounded(12)
                    .width(2)
                    .color(palette.danger.strong.color),
            )
    }

    fn create_message(&mut self) {
        let text = self.input_message.clone();
        self.chat.push(Message::new(self.user.clone(), text));
        self.input_message.clear();
    }

    fn get_response(&self, messages: Vec<ChatMessage>) -> Task<AppCommand> {
        let llm = self.settings.llm(self.char.clone());
        println!("Getting response with chat history:");
        for mes in &messages {
            println!("{:?}", mes);
        }
        println!();
        Task::perform(async move { llm.chat_stream(&messages).await }, |res| res).and_then(|res| {
            Task::run(res, |chunk| match chunk {
                Ok(text) => AppCommand::StreamOk(text),
                Err(e) => AppCommand::Error(e.to_string()),
            })
        })
    }
}
