use std::time::Duration;

use iced::{
    Border, Element,
    Length::{self, Fill},
    Subscription, Task, Theme, time,
    widget::{Row, Stack, column, container, row},
};
use iced_modern_theme::Modern;
use log::trace;
use tokio::time::sleep;

use crate::{
    char_selector_page::CharSelectorPage,
    chat_page::{ChatCommand, ChatPage},
    settings_page::{SettingsChange, SettingsPage},
    utils::widgets::{button, text},
};

mod app_settings;
mod char_selector_page;
mod chat_page;
mod formater;
mod settings_page;
mod utils;

pub fn main() -> iced::Result {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Off) // Default: everything off
        .filter_module("fullmoon", log::LevelFilter::Trace)
        .filter_module("libmoon", log::LevelFilter::Trace)
        .filter_module("llm", log::LevelFilter::Trace)
        .init();

    iced::application("FullMoon", App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .run_with(|| (App::new(), iced::Task::none()))
}

struct App {
    chat_page: ChatPage,
    char_selector_page: Option<CharSelectorPage>,
    settings_page: SettingsPage,
    show_settings: bool,
    error: Option<String>,
}

#[derive(Debug, Clone)]
enum AppCommand {
    ChatCommand(ChatCommand),

    ToggleChars,
    SelectedChar(usize),

    ToggleSettings,
    SettignsCommand(SettingsChange),

    Error(String),
    DismissError,

    Update,
}

impl App {
    fn new() -> Self {
        let chat_page = ChatPage::try_load();
        let chat_settings = chat_page.chat.settings().clone();
        App {
            chat_page,
            char_selector_page: None,
            settings_page: SettingsPage::load(chat_settings),
            show_settings: false,
            error: None,
        }
    }

    fn update(&mut self, message: AppCommand) -> Task<AppCommand> {
        match message {
            AppCommand::ChatCommand(chat_command) => {
                return self.chat_page.update(chat_command);
            }
            AppCommand::ToggleChars => {
                self.char_selector_page = match self.char_selector_page {
                    None => {
                        trace!("Opening Char selector page");
                        Some(CharSelectorPage::new())
                    }
                    Some(_) => {
                        trace!("Closing Char selector page");
                        None
                    }
                };
            }
            AppCommand::SelectedChar(char_idx) => {
                if let Some(csp) = &mut self.char_selector_page {
                    let char = csp.get(char_idx);
                    trace!("Selected {}", char.name());
                    self.chat_page.set_char(char)
                }
            }
            AppCommand::ToggleSettings => {
                self.show_settings = match self.show_settings {
                    false => {
                        trace!("Opening settings page");
                        true
                    }
                    true => {
                        trace!("Closing settings page");
                        false
                    }
                };
            }
            AppCommand::SettignsCommand(settings_command) => {
                self.settings_page.update(settings_command)
            }
            AppCommand::Error(e) => {
                self.error = Some(e);
                return Task::perform(sleep(Duration::from_secs(3)), |_| AppCommand::DismissError);
            }
            AppCommand::DismissError => self.error = None,
            AppCommand::Update => (),
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, AppCommand> {
        let mut pages = Row::new().spacing(20);
        if let Some(char_selector_page) = &self.char_selector_page {
            pages = pages.push(char_selector_page.view(&self.settings_page))
        }
        if self.show_settings {
            pages = pages.push(self.settings_page.view())
        }
        pages = pages.push(self.chat_page.view(&self.settings_page));

        let mut stack = Stack::new();
        stack = stack.push(column![
            row![
                button("User", &self.settings_page).width(Fill),
                button("Characters", &self.settings_page)
                    .on_press(AppCommand::ToggleChars)
                    .width(Fill),
                button("Settings", &self.settings_page)
                    .on_press(AppCommand::ToggleSettings)
                    .width(Fill)
            ]
            .padding(20)
            .spacing(20),
            pages
        ]);
        if let Some(e) = &self.error {
            stack = stack.push(
                container(
                    container(text(e, &self.settings_page))
                        .padding(20)
                        .style(Self::error_style),
                )
                .center_x(Length::Fill)
                .padding(20),
            );
        }
        stack.into()
    }

    fn theme(&self) -> Theme {
        Modern::theme(true)
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

    fn subscription(&self) -> Subscription<AppCommand> {
        time::every(Duration::from_millis(16)).map(|_| AppCommand::Update)
    }
}
