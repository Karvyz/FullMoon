use dirs::config_dir;
use iced::{
    Alignment, Border, Element, Font,
    Length::Fill,
    Theme,
    font::Weight,
    widget::{checkbox, column, container, slider, text, text_input},
};
use iced_modern_theme::colors::colors;
use llm::{
    LLMProvider,
    builder::{LLMBackend, LLMBuilder},
};
use log::{error, trace};
use serde::{Deserialize, Serialize};
use std::{fs, sync::Arc};

use crate::{AppCommand, persona::Persona};

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
    reasoning: bool,
    font_size: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: "sk-TESTKEY".to_string(),
            model: "google/gemma-3-27b-it".to_string(),
            temperature: 0.5,
            max_tokens: 1000,
            reasoning: false,
            font_size: 16.0,
        }
    }
}

impl Settings {
    pub fn llm(&self, char: &Arc<Persona>, user: &Arc<Persona>) -> Box<dyn LLMProvider> {
        LLMBuilder::new()
            .backend(LLMBackend::OpenRouter)
            .api_key(self.api_key.clone())
            .model(self.model.clone())
            .temperature(self.temperature)
            .max_tokens(self.max_tokens)
            .reasoning(self.reasoning)
            .system(char.system_prompt(Some(&user.name())))
            .build()
            .expect("Failed to build LLM (Openrouter)")
    }

    pub fn load() -> Self {
        trace!("Loading config started");
        let path = config_dir()
            .map(|mut path| {
                path.push("fullmoon");
                path.push("settings.json");
                path
            })
            .unwrap();

        match path.exists() {
            true => match fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(settings) => {
                        trace!("Loading config finished");
                        settings
                    }
                    Err(e) => {
                        error!("Error parsing config: {}", e);
                        Self::default()
                    }
                },
                Err(e) => {
                    error!("Error reading config: {}", e);
                    Self::default()
                }
            },
            false => {
                let default = Self::default();
                error!("Config not found. Writing default");
                default.save().unwrap_or_else(|e| error!("{e}"));
                default
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = config_dir().ok_or("Unable to find config directory")?;

        let fullmoon_dir = config_dir.join("fullmoon");
        if !fullmoon_dir.exists() {
            fs::create_dir_all(&fullmoon_dir)?;
        }

        let config_path = fullmoon_dir.join("settings.json");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;

        Ok(())
    }

    pub fn view(&self) -> Element<'_, AppCommand> {
        container(
            column![
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
                container(
                    column![
                        text("App settings").font(Font {
                            weight: Weight::Bold,
                            ..Default::default()
                        }),
                        column![
                            text(format! {"Font size: {}", self.font_size}),
                            slider(4.0..=100.0, self.font_size, |fs| {
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
                self.api_key = key
            }
            SettingsChange::Model(model) => {
                trace!("Update model: {model}");
                self.model = model
            }
            SettingsChange::Temperature(temperature) => {
                trace!("Update temperature: {temperature}");
                self.temperature = temperature
            }
            SettingsChange::MaxTokens(max_tokens) => {
                trace!("Update max_tokens: {max_tokens}");
                self.max_tokens = max_tokens
            }
            SettingsChange::Reasoning(reasoning) => {
                trace!("Update reasoning: {reasoning}");
                self.reasoning = reasoning
            }
            SettingsChange::FontSize(font_size) => {
                trace!("Update font size: {font_size}");
                self.font_size = font_size
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
