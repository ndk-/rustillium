use eframe::egui::{self, CentralPanel, ViewportBuilder, ViewportId};
use std::collections::HashMap;
use crate::credentials_provider::{CredentialsProvider};

pub struct AddEditDialog {
    credentials_provider: CredentialsProvider,
    secret_name: String,
    dialog_secrets: HashMap<String, String>,
    open_dialog: bool,
}

impl AddEditDialog {
    pub fn new(credentials_provider: &CredentialsProvider) -> Self {
        Self {
            credentials_provider: credentials_provider.clone(),
            secret_name: "".to_string(),
            dialog_secrets: HashMap::new(),
            open_dialog: false,
        }
    }

    pub fn open(&mut self, secret_name: &str) {
        self.secret_name = secret_name.to_string();
        if secret_name.is_empty() {
            self.dialog_secrets.insert("username".to_string(), "".to_string());
            self.dialog_secrets.insert("password".to_string(), "".to_string());
        } else {
            self.dialog_secrets = self.credentials_provider.load_secrets(secret_name).expect(format!("cannot load secret {}", secret_name).as_str());
        }
        
        self.open_dialog = true;
    }

    fn close(&mut self) {
        self.open_dialog = false;
        self.secret_name = "".to_string();
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if self.open_dialog {
            let secrets_dialog = ViewportBuilder::default()
                .with_title("add a new secret")
                .with_close_button(true)
                .with_decorations(true);
            let dialog_id = ViewportId::from_hash_of("secrets_dialog");

            ctx.show_viewport_immediate(dialog_id, secrets_dialog, |ctx, _| {
                if ctx.input(|input_state| input_state.viewport().close_requested()) {
                    self.close();
                }
                CentralPanel::default().show(ctx, |ui| {
                    if ui.button("close").clicked() {
                        self.close();
                    }
                });
            });
        }
    }
}