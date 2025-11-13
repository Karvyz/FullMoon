use iced::{
    Alignment, Border, Element, Font,
    Length::Fill,
    Task, Theme,
    font::Weight,
    widget::{checkbox, column, container, slider, text, text_input},
};
use iced_modern_theme::colors::colors;

use crate::{AppCommand, settings::Settings};

#[derive(Debug, Clone)]
pub enum SettingsChange {
    ApiKey(String),
    Model(String),
    Temperature(f32),
    MaxTokens(u32),
    Reasoning(bool),
}

impl From<SettingsChange> for crate::AppCommand {
    fn from(settings_command: SettingsChange) -> Self {
        crate::AppCommand::SettignsCommand(settings_command)
    }
}

pub struct SettingsPage {
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
    reasoning: bool,
}

impl SettingsPage {
    pub fn new(settings: &Settings) -> Self {
        SettingsPage {
            api_key: settings.api_key(),
            model: settings.model(),
            temperature: settings.temperature(),
            max_tokens: settings.max_tokens(),
            reasoning: settings.reasoning(),
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
                            .on_input(|t| SettingsChange::ApiKey(t).into())
                            .on_paste(|t| SettingsChange::ApiKey(t).into())
                            .secure(true)
                            .width(Fill)
                    ]
                    .spacing(5),
                    column![
                        text("Model:"),
                        text_input("google/gemma-3-27b-it", &self.model)
                            .on_input(|t| SettingsChange::Model(t).into())
                            .on_paste(|t| SettingsChange::Model(t).into())
                            .width(Fill)
                    ]
                    .spacing(5),
                    column![
                        text(format! {"Temperature: {}", self.temperature}),
                        slider(0.0..=1.0, self.temperature, |t| {
                            SettingsChange::Temperature(t).into()
                        })
                        .step(0.01)
                        .width(Fill)
                    ]
                    .spacing(5),
                    column![
                        text(format! {"Max tokens: {}", self.max_tokens}),
                        slider(0..=10000, self.max_tokens, |mt| {
                            SettingsChange::MaxTokens(mt).into()
                        })
                        .width(Fill),
                    ]
                    .spacing(5),
                    checkbox("Reasoning", self.reasoning)
                        .on_toggle(|r| SettingsChange::Reasoning(r).into()),
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

    pub fn update(&mut self, settings_command: SettingsChange) -> Task<AppCommand> {
        match settings_command {
            SettingsChange::ApiKey(key) => self.api_key = key,
            SettingsChange::Model(model) => self.model = model,
            SettingsChange::Temperature(temperature) => self.temperature = temperature,
            SettingsChange::MaxTokens(max_tokens) => self.max_tokens = max_tokens,
            SettingsChange::Reasoning(reasoning) => self.reasoning = reasoning,
        }

        let new_settings = Settings::new(
            self.api_key.clone(),
            self.model.clone(),
            self.temperature,
            self.max_tokens,
            self.reasoning,
        );
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
