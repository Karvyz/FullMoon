use iced::{
    keyboard::{self, key},
    widget::text_editor::{Binding, KeyPress, Status},
};

use crate::{AppCommand, chat_page::ChatCommand};

pub fn from_key_press(event: KeyPress) -> Option<Binding<AppCommand>> {
    if event.status == Status::Focused
        && let keyboard::Key::Named(key::Named::Enter) = event.key.as_ref()
    {
        return match event.modifiers.shift() {
            true => Some(Binding::Enter),
            false => Some(Binding::Custom(AppCommand::ChatCommand(
                ChatCommand::InputSubmit,
            ))),
        };
    }

    Binding::from_key_press(event)
}
