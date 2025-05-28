#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use crate::credentials_provider::CredentialsProvider;
use eframe::egui::{self, Button, TextEdit};
use std::collections::HashMap;

pub struct AppUI {
    credentials_provider: CredentialsProvider,
}

impl AppUI {
    pub fn new(credentials_provider: CredentialsProvider) -> Self {
        Self {
            credentials_provider,
        }
    }

    pub fn show(self: Self) -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
        };

        let mut search_term = "".to_owned();

        eframe::run_simple_native("Rustillium", options, move |ctx, _frame| {
            let secrets = self
                .credentials_provider
                .load_secret_names()
                .expect("Failed to load secret names");
            egui::CentralPanel::default().show(ctx, |ui| {
                self.build_search_field(ui, &mut search_term);
                self.build_secrets_section(ui, &search_term, secrets);
            });
        })
    }

    fn build_secrets_section(&self, ui: &mut egui::Ui, search_term: &String, secrets: Vec<String>) {
        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                let compared_term = search_term.clone();
                secrets
                    .iter()
                    .filter(|secret| secret.to_string().contains(compared_term.as_str()))
                    .for_each(|secret| {
                        self.build_secret_section(ui, secret);
                    });
            });
    }

    fn build_secret_section(&self, ui: &mut egui::Ui, secret: &String) {
        ui.collapsing(secret.to_string(), |ui| {
            let mut maybe_secrets: Option<Vec<(String, String)>> =
                ui.data(|reader| return reader.get_temp(ui.id()));

            if maybe_secrets == None {
                let secrets = AppUI::to_secrets_array(
                    self.credentials_provider
                        .load_secrets(secret)
                        .expect("Cannot load secrets file"),
                );
                maybe_secrets = Some(secrets.clone());
                ui.data_mut(|writer| {
                    writer.insert_temp(ui.id(), secrets);
                });
            }

            maybe_secrets.unwrap().iter().for_each(|(key, value)| {
                ui.horizontal(|ui| {
                    ui.label(key);
                    if ui
                        .add_sized(ui.available_size(), Button::new(value))
                        .clicked()
                    {
                        ui.ctx().copy_text(value.to_string());
                    };
                });
            });
        });
    }

    fn build_search_field(&self, ui: &mut egui::Ui, search_term: &mut String) {
        ui.horizontal(|ui| {
            ui.label("Search: ");
            ui.add_sized(
                ui.available_size(),
                TextEdit::singleline(search_term).hint_text("search by secret name"),
            );
        });
    }

    fn to_secrets_array(secrets: HashMap<String, String>) -> Vec<(String, String)> {
        let mut result: Vec<(String, String)> = Vec::new();
        result.push((
            "login".to_string(),
            secrets.get("login").unwrap_or(&"".to_string()).to_owned(),
        ));
        result.push((
            "password".to_string(),
            secrets
                .get("password")
                .unwrap_or(&"".to_string())
                .to_owned(),
        ));
        secrets
            .iter()
            .filter(|item| item.0 != "login" && item.0 != "password")
            .for_each(|item| result.push((item.0.clone(), item.1.clone())));
        return result;
    }
}
