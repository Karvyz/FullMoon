mod chat;
mod message;

use std::sync::Arc;

use chat::Chat;
use iced::widget::{Column, column, text_input};
use iced::{Center, Task, Theme};
use llm::LLMProvider;
use llm::builder::{LLMBackend, LLMBuilder};

use crate::message::{Message, MessageOwner};

pub fn main() -> iced::Result {
    iced::application("FullMoon", App::update, App::view)
        .theme(App::theme)
        .run_with(|| (App::new(), iced::Task::none()))
}

struct App {
    input_message: String,
    chat: Chat,
    llm: Arc<Box<dyn LLMProvider>>,
}

#[derive(Debug, Clone)]
enum IcedMessage {
    InputChange(String),
    CreateMessage,
    AddMessage(Message),
    ErrorMessage(String),
}

impl App {
    fn new() -> Self {
        let api_key = std::env::var("OPENROUTER_API_KEY").unwrap_or("sk-TESTKEY".into());
        println!("Api key: {}", api_key);
        App {
            input_message: String::new(),
            chat: Chat::default(),
            llm: Arc::new(
                LLMBuilder::new()
                    .backend(LLMBackend::OpenRouter)
                    .api_key(api_key)
                    .model("google/gemma-3-27b-it")
                    .build()
                    .expect("Failed to build LLM (Openrouter)"),
            ),
        }
    }

    fn update(&mut self, message: IcedMessage) -> Task<IcedMessage> {
        match message {
            IcedMessage::InputChange(input) => self.input_message = input,
            IcedMessage::CreateMessage => {
                self.create_message();
                return Task::perform(
                    Message::get_response(self.llm.clone(), self.chat.get_messages()),
                    |resp| match resp {
                        Ok(msg) => IcedMessage::AddMessage(msg),
                        Err(e) => IcedMessage::ErrorMessage(e),
                    },
                );
            }
            IcedMessage::AddMessage(message) => self.chat.push(message),
            IcedMessage::ErrorMessage(e) => println!("{}", e),
        }
        Task::none()
    }

    fn view(&self) -> Column<'_, IcedMessage> {
        column![
            self.chat.view(),
            text_input("What needs to be done?", &self.input_message)
                .id("user-input")
                .on_input(IcedMessage::InputChange)
                .on_submit(IcedMessage::CreateMessage),
        ]
        .padding(20)
        .align_x(Center)
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    fn create_message(&mut self) {
        let text = self.input_message.clone();
        self.chat.push(Message::new(MessageOwner::User, text));
        self.input_message.clear();
    }
}
