use std::time::Duration;

use iced::{
    Border, Element, Length, Task, Theme,
    widget::{Stack, container, text},
};
use tokio::time::sleep;

use crate::{
    chat_page::{ChatCommand, ChatPage},
    settings::Settings,
};

mod chat_page;
mod message;
mod persona;
mod settings;

pub fn main() -> iced::Result {
    iced::application("FullMoon", App::update, App::view)
        .theme(App::theme)
        .run_with(|| (App::new(), iced::Task::none()))
}

struct App {
    chat_page: ChatPage,
    settings: Settings,
    error: Option<String>,
}

#[derive(Debug, Clone)]
enum AppCommand {
    ChatCommand(ChatCommand),
    Error(String),
    DismissError,
}

impl App {
    fn new() -> Self {
        App {
            chat_page: ChatPage::new(),
            settings: Settings::load(),
            error: None,
        }
    }

    fn update(&mut self, message: AppCommand) -> Task<AppCommand> {
        match message {
            AppCommand::ChatCommand(chat_command) => {
                return self.chat_page.update(chat_command, &self.settings);
            }
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
        stack = stack.push(self.chat_page.view());
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
}
