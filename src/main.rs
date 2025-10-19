mod chat;

use chat::{Chat, Message, MessageOwner};
use iced::widget::{Column, column, text_input};
use iced::{Center, Task, Theme};
use llm::chat::ChatResponse;
use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::ChatMessage,
};

pub fn main() -> iced::Result {
    iced::application("FullMoon", App::update, App::view)
        .theme(App::theme)
        .run()
}

#[derive(Default)]
struct App {
    input_message: String,
    chat: Chat,
}

#[derive(Debug, Clone)]
enum IcedMessage {
    InputChange(String),
    CreateMessage,
    ResponseSucces(String),
    ResponseError(String),
}

// Helper implementation for converting Result to Message
impl IcedMessage {
    fn from_result(result: Result<Box<dyn ChatResponse>, llm::error::LLMError>) -> Self {
        match result {
            Ok(data) => IcedMessage::ResponseSucces(data.to_string()),
            Err(err) => IcedMessage::ResponseError(err.to_string()),
        }
    }
}

impl App {
    fn update(&mut self, message: IcedMessage) -> Task<IcedMessage> {
        match message {
            IcedMessage::InputChange(input) => self.input_message = input,
            IcedMessage::CreateMessage => {
                self.create_message();
                return Task::perform(
                    get_response(self.chat.get_messages()),
                    IcedMessage::from_result,
                );
            }
            IcedMessage::ResponseSucces(response) => {
                self.chat.push(Message::new(MessageOwner::Char, response))
            }
            IcedMessage::ResponseError(error) => println!("{}", error),
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

async fn get_response(
    messages: Vec<Message>,
) -> Result<Box<dyn ChatResponse>, llm::error::LLMError> {
    let api_key = std::env::var("OPENROUTER_API_KEY").unwrap_or("sk-TESTKEY".into());
    println!("Api key: {}", api_key);

    // Initialize and configure the LLM client with streaming enabled
    let llm = LLMBuilder::new()
        .backend(LLMBackend::OpenRouter)
        .api_key(api_key)
        .model("google/gemma-3-27b-it")
        .build()
        .expect("Failed to build LLM (Openrouter)");

    let mut chat_messages = vec![];
    for msg in &messages {
        chat_messages.push(match msg.owner {
            MessageOwner::User => ChatMessage::user().content(msg.text.clone()).build(),
            MessageOwner::Char => ChatMessage::assistant().content(msg.text.clone()).build(),
        })
    }

    println!("Starting chat with Openrouter...\n");
    for mes in &chat_messages {
        println!("{:?}", mes);
    }

    llm.chat(&chat_messages).await
}
