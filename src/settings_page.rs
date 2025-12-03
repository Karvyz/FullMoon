use std::ops::Deref;

use iced::{
    Alignment, Border, Element,
    Length::Fill,
    Theme,
    widget::{checkbox, column, container, slider, text_input},
};
use iced_modern_theme::colors::colors;
use log::{error, trace};

use crate::{
    AppCommand,
    app_settings::AppSettings,
    utils::widgets::{bold_text, text},
};

#[derive(Debug, Clone)]
pub enum SettingsChange {
    ApiKey(String),
    Model(String),
    Temperature(f32),
    MaxTokens(u32),
    Reasoning(bool),
    FontSize(f32),
}

impl From<SettingsChange> for crate::AppCommand {
    fn from(settings_command: SettingsChange) -> Self {
        crate::AppCommand::SettignsCommand(settings_command)
    }
}

#[derive(Debug, Default, Clone)]
pub struct SettingsPage {
    app: AppSettings,
    chat: libmoon::settings::Settings,
}

impl Deref for SettingsPage {
    type Target = libmoon::settings::Settings;

    fn deref(&self) -> &Self::Target {
        &self.chat
    }
}

impl SettingsPage {
    pub fn font_size(&self) -> f32 {
        self.app.font_size
    }

    pub fn load(chat_settings: libmoon::settings::Settings) -> Self {
        SettingsPage {
            app: AppSettings::load(),
            chat: chat_settings,
        }
    }

    pub fn view(&self) -> Element<'_, AppCommand> {
        container(
            column![
                container(
                    column![
                        bold_text("API settings", self),
                        column![
                            text("API Key:", self),
                            text_input("sk-************************************", &self.api_key)
                                .size(self.app.font_size)
                                .on_input(|t| SettingsChange::ApiKey(t).into())
                                .on_paste(|t| SettingsChange::ApiKey(t).into())
                                .secure(true)
                                .width(Fill)
                        ]
                        .spacing(5),
                        column![
                            text("Model:", self),
                            text_input("google/gemma-3-27b-it", &self.model)
                                .size(self.app.font_size)
                                .on_input(|t| SettingsChange::Model(t).into())
                                .on_paste(|t| SettingsChange::Model(t).into())
                                .width(Fill)
                        ]
                        .spacing(5),
                        column![
                            text(format! {"Temperature: {}", self.temperature}, self),
                            slider(0.0..=1.0, self.temperature, |t| {
                                SettingsChange::Temperature(t).into()
                            })
                            .step(0.01)
                            .width(Fill)
                        ]
                        .spacing(5),
                        column![
                            text(format! {"Max tokens: {}", self.max_tokens}, self),
                            slider(0..=10000, self.max_tokens, |mt| {
                                SettingsChange::MaxTokens(mt).into()
                            })
                            .width(Fill),
                        ]
                        .spacing(5),
                        checkbox("Reasoning", self.reasoning)
                            .size(self.app.font_size)
                            .on_toggle(|r| SettingsChange::Reasoning(r).into()),
                    ]
                    .align_x(Alignment::Center)
                    .spacing(10)
                    .padding(10),
                )
                .style(Self::box_style)
                .padding(10),
                container(
                    column![
                        bold_text("App settings", self),
                        column![
                            text(format! {"Font size: {}", self.app.font_size}, self),
                            slider(4.0..=100.0, self.app.font_size, |fs| {
                                SettingsChange::FontSize(fs).into()
                            })
                            .width(Fill),
                        ]
                        .spacing(5),
                    ]
                    .align_x(Alignment::Center)
                    .spacing(10)
                    .padding(10),
                )
                .style(Self::box_style)
                .padding(10)
            ]
            .spacing(10),
        )
        .padding(10)
        .width(Fill)
        .into()
    }

    pub fn update(&mut self, settings_command: SettingsChange) {
        match settings_command {
            SettingsChange::ApiKey(key) => {
                trace!("Update key");
                self.chat.api_key = key
            }
            SettingsChange::Model(model) => {
                trace!("Update model: {model}");
                self.chat.model = model
            }
            SettingsChange::Temperature(temperature) => {
                trace!("Update temperature: {temperature}");
                self.chat.temperature = temperature
            }
            SettingsChange::MaxTokens(max_tokens) => {
                trace!("Update max_tokens: {max_tokens}");
                self.chat.max_tokens = max_tokens
            }
            SettingsChange::Reasoning(reasoning) => {
                trace!("Update reasoning: {reasoning}");
                self.chat.reasoning = reasoning
            }
            SettingsChange::FontSize(font_size) => {
                trace!("Update font size: {font_size}");
                self.app.font_size = font_size;
                self.app.save();
            }
        }

        if let Err(e) = self.save() {
            error!("{e}")
        }
    }

    fn box_style(theme: &Theme) -> iced::widget::container::Style {
        container::rounded_box(theme)
            .background(colors::fill::SECONDARY_DARK)
            .border(Border::default().rounded(12))
    }
}
