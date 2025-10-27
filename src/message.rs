use std::sync::Arc;

use llm::{LLMProvider, chat::ChatMessage};

#[derive(Debug, Clone)]
pub enum MessageOwner {
    User,
    Char,
}

#[derive(Debug, Clone)]
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

    pub async fn get_response(
        llm: Arc<Box<dyn LLMProvider>>,
        messages: Vec<Message>,
    ) -> Result<Self, String> {
        // Initialize and configure the LLM client with streaming enabled
        let mut chat_messages = vec![];
        for msg in &messages {
            chat_messages.push(msg.to_chat_message())
        }

        println!("Starting chat with Openrouter...\n");
        for mes in &chat_messages {
            println!("{:?}", mes);
        }

        let resp = llm.chat(&chat_messages).await;
        match resp {
            Ok(text) => Ok(Message::new(MessageOwner::Char, text.to_string())),
            Err(e) => Err(e.to_string()),
        }
    }

    fn to_chat_message(&self) -> ChatMessage {
        match self.owner {
            MessageOwner::User => ChatMessage::user().content(self.text.clone()).build(),
            MessageOwner::Char => ChatMessage::assistant().content(self.text.clone()).build(),
        }
    }
}
