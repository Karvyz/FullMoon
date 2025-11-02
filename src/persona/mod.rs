use llm::chat::ChatMessage;

pub mod char;
pub mod user;

pub trait Persona {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_avatar_uri(&self) -> String;

    fn create_message(&self, text: String) -> ChatMessage;
}
