use llm::chat::ChatMessage;

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

    pub fn empty(owner: MessageOwner) -> Self {
        Message {
            owner,
            text: String::new(),
        }
    }

    pub fn to_chat_message(&self) -> ChatMessage {
        match self.owner {
            MessageOwner::User => ChatMessage::user().content(self.text.clone()).build(),
            MessageOwner::Char => ChatMessage::assistant().content(self.text.clone()).build(),
        }
    }
}
