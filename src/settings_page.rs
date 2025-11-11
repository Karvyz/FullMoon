use iced::{
    Alignment, Border, Element, Font,
    Length::Fill,
    Task, Theme,
    font::Weight,
    widget::{column, container, text, text_input},
};
use iced_modern_theme::colors::colors;

use crate::{AppCommand, settings::Settings};

#[derive(Debug, Clone)]
pub enum SettingsCommand {
    ApiKeyChange(String),
    ModelChange(String),
}

impl From<SettingsCommand> for crate::AppCommand {
    fn from(settings_command: SettingsCommand) -> Self {
        crate::AppCommand::SettignsCommand(settings_command)
    }
}

pub struct SettingsPage {
    api_key: String,
    model: String,
}

impl SettingsPage {
    pub fn new(settings: &Settings) -> Self {
        SettingsPage {
            api_key: settings.api_key(),
            model: settings.model(),
        }
    }

    pub fn view(&self) -> Element<'_, AppCommand> {
        container(
            container(
                column![
                    text("API settings").font(Font {
                        weight: Weight::Bold,
                        ..Default::default()
                    }),
                    column![
                        text("API Key:"),
                        text_input("sk-************************************", &self.api_key)
                            .on_input(|t| SettingsCommand::ApiKeyChange(t).into())
                            .on_paste(|t| SettingsCommand::ApiKeyChange(t).into())
                            .secure(true)
                            .width(Fill)
                    ]
                    .spacing(5),
                    column![
                        text("Model:"),
                        text_input("google/gemma-3-27b-it", &self.model)
                            .on_input(|t| SettingsCommand::ModelChange(t).into())
                            .on_paste(|t| SettingsCommand::ModelChange(t).into())
                            .width(Fill)
                    ]
                    .spacing(5),
                ]
                .align_x(Alignment::Center)
                .spacing(10)
                .padding(10),
            )
            .style(Self::box_style)
            .padding(10),
        )
        .padding(10)
        .width(Fill)
        .into()
    }

    pub fn update(&mut self, settings_command: SettingsCommand) -> Task<AppCommand> {
        match settings_command {
            SettingsCommand::ApiKeyChange(key) => self.api_key = key,
            SettingsCommand::ModelChange(model) => self.model = model,
        }

        let new_settings = Settings::new(self.api_key.clone(), self.model.clone());
        Task::done(match new_settings.save() {
            Ok(_) => AppCommand::UpdateSettings(new_settings),
            Err(e) => AppCommand::Error(e.to_string()),
        })
    }

    fn box_style(theme: &Theme) -> iced::widget::container::Style {
        container::rounded_box(theme)
            .background(colors::fill::SECONDARY_DARK)
            .border(Border::default().rounded(12))
    }
}
