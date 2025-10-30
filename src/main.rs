mod chat;
mod message;
mod settings;

use std::sync::Arc;

use chat::Chat;
use iced::widget::{Column, column, text_input};
use iced::{Center, Task, Theme};
use llm::LLMProvider;
use llm::chat::ChatMessage;

use crate::chat::MessageCommand;
use crate::message::{Message, MessageOwner};
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
    llm: Arc<Box<dyn LLMProvider>>,
}

#[derive(Debug, Clone)]
enum AppCommand {
    InputChange(String),
    CreateMessage,
    StreamOk(String),
    StreamError,
    MessageCommand(usize, MessageCommand),
}

impl App {
    fn new() -> Self {
        let settings = Settings::load();
        let llm = settings.llm();
        App {
            input_message: String::new(),
            chat: Chat::default(),
            settings,
            llm: Arc::new(llm),
        }
    }

    fn update(&mut self, message: AppCommand) -> Task<AppCommand> {
        match message {
            AppCommand::InputChange(input) => self.input_message = input,
            AppCommand::CreateMessage => {
                self.create_message();
                let chat_history = self.chat.get_chat_messages();
                self.chat.push(Message::empty(MessageOwner::Char));
                return self.get_response(&self.llm, chat_history);
            }
            AppCommand::StreamOk(text) => self.chat.append_last_message(text.as_str()),
            AppCommand::StreamError => todo!(),
            AppCommand::MessageCommand(idx, message_command) => {
                println!("Command {:?} from {idx}", message_command);
                match message_command {
                    MessageCommand::Next => {
                        if self.chat.next(idx) {
                            return self
                                .get_response(&self.llm, self.chat.get_chat_messages_until(idx));
                        }
                    }
                    MessageCommand::Previous => self.chat.previous(idx),
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Column<'_, AppCommand> {
        column![
            self.chat.view(),
            text_input("What needs to be done?", &self.input_message)
                .id("user-input")
                .on_input(AppCommand::InputChange)
                .on_submit(AppCommand::CreateMessage),
        ]
        .padding(20)
        .align_x(Center)
        .spacing(10)
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    fn create_message(&mut self) {
        let text = self.input_message.clone();
        self.chat.push(Message::new(MessageOwner::User, text));
        self.input_message.clear();
    }

    fn get_response(
        &self,
        llm: &Arc<Box<dyn LLMProvider>>,
        messages: Vec<ChatMessage>,
    ) -> Task<AppCommand> {
        println!("Getting response with chat history:");
        for mes in &messages {
            println!("{:?}", mes);
        }
        println!();
        let llm = llm.clone();
        Task::perform(async move { llm.chat_stream(&messages).await }, |res| res).and_then(|res| {
            Task::run(res, |chunk| match chunk {
                Ok(text) => AppCommand::StreamOk(text),
                Err(_) => AppCommand::StreamError,
            })
        })
    }
}
