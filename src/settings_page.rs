use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{column, text, text_input},
};

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
        column![
            text("API Key:"),
            text_input("sk-************************************", &self.api_key)
                .on_input(|t| SettingsCommand::ApiKeyChange(t).into())
                .on_paste(|t| SettingsCommand::ApiKeyChange(t).into())
                .width(Fill),
            text("Model:"),
            text_input("google/gemma-3-27b-it", &self.model)
                .on_input(|t| SettingsCommand::ModelChange(t).into())
                .on_paste(|t| SettingsCommand::ModelChange(t).into())
                .width(Fill)
        ]
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
}
