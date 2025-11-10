use std::time::Duration;

use iced::{
    Border, Element,
    Length::{self, Fill},
    Task, Theme,
    widget::{Row, Stack, button, column, container, row, text},
};
use settings_page::{SettingsCommand, SettingsPage};
use tokio::time::sleep;

use crate::{
    char_selector_page::CharSelectorPage,
    chat_page::{ChatCommand, ChatPage},
    settings::Settings,
};

mod char_selector_page;
mod chat_page;
mod message;
mod persona;
mod settings;
mod settings_page;

pub fn main() -> iced::Result {
    iced::application("FullMoon", App::update, App::view)
        .theme(App::theme)
        .run_with(|| (App::new(), iced::Task::none()))
}

struct App {
    chat_page: ChatPage,
    char_selector_page: Option<CharSelectorPage>,
    settings_page: Option<SettingsPage>,
    settings: Settings,
    error: Option<String>,
}

#[derive(Debug, Clone)]
enum AppCommand {
    ChatCommand(ChatCommand),

    ToggleChars,
    SelectedChar(usize),

    ToggleSettings,
    SettignsCommand(SettingsCommand),
    UpdateSettings(Settings),

    Error(String),
    DismissError,
}

impl App {
    fn new() -> Self {
        App {
            chat_page: ChatPage::try_load(),
            char_selector_page: None,
            settings_page: None,
            settings: Settings::load(),
            error: None,
        }
    }

    fn update(&mut self, message: AppCommand) -> Task<AppCommand> {
        match message {
            AppCommand::ChatCommand(chat_command) => {
                return self.chat_page.update(chat_command, &self.settings);
            }
            AppCommand::ToggleChars => {
                self.char_selector_page = match self.char_selector_page {
                    None => Some(CharSelectorPage::new()),
                    Some(_) => None,
                };
            }
            AppCommand::SelectedChar(char_idx) => {
                if let Some(csp) = &self.char_selector_page {
                    self.chat_page.set_char(csp.get(char_idx))
                }
            }
            AppCommand::ToggleSettings => {
                self.settings_page = match self.settings_page {
                    None => Some(SettingsPage::new(&self.settings)),
                    Some(_) => None,
                };
            }
            AppCommand::SettignsCommand(settings_command) => {
                if let Some(settings_page) = &mut self.settings_page {
                    return settings_page.update(settings_command);
                }
            }
            AppCommand::UpdateSettings(settings) => self.settings = settings,
            AppCommand::Error(e) => {
                self.error = Some(e);
                return Task::perform(sleep(Duration::from_secs(3)), |_| AppCommand::DismissError);
            }
            AppCommand::DismissError => self.error = None,
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, AppCommand> {
        let mut pages = Row::new().spacing(20);
        if let Some(char_selector_page) = &self.char_selector_page {
            pages = pages.push(char_selector_page.view())
        }
        if let Some(settings_page) = &self.settings_page {
            pages = pages.push(settings_page.view())
        }
        pages = pages.push(self.chat_page.view());

        let mut stack = Stack::new();
        stack = stack.push(column![
            row![
                button("User").width(Fill),
                button("Characters")
                    .width(Fill)
                    .on_press(AppCommand::ToggleChars),
                button("Settings")
                    .width(Fill)
                    .on_press(AppCommand::ToggleSettings)
            ]
            .padding(20)
            .spacing(20),
            pages
        ]);
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
