use std::rc::Rc;

use crate::credentials_provider::CredentialsProvider;
use eframe::egui::{Align, CentralPanel, Context, Id, Layout, TopBottomPanel, Ui, ViewportBuilder, ViewportId};

const ADD_SECRET_TITLE: &str = "Add New Secret";
const MODIFY_SECRET_TITLE: &str = "Modify Secret";
const CREDENTIAL_FIELDS: &[&str] = &["username", "password"];
const DELETE_BUTTON_LABEL: &str = " \u{1F5D1}";
const ADD_BUTTON_LABEL: &str = "\u{2795} Add";
const CANCEL_BUTTON_LABEL: &str = "\u{274c} Cancel";
const SAVE_BUTTON_LABEL: &str = "\u{1f4be} Save";

pub struct ModifySecretUI {
    credentials_provider: Rc<CredentialsProvider>,
    updated_secret_name: String,
    original_secret_name: String,
    dialog_secrets: Vec<(String, String)>,
    open_dialog: bool,
    title: String,
    error_message: Option<String>,
}

impl ModifySecretUI {
    pub fn new(credentials_provider: &Rc<CredentialsProvider>) -> Self {
        Self {
            credentials_provider: Rc::clone(credentials_provider),
            updated_secret_name: "".to_string(),
            original_secret_name: "".to_string(),
            dialog_secrets: Vec::new(),
            open_dialog: false,
            title: ADD_SECRET_TITLE.to_string(),
            error_message: None,
        }
    }

    pub fn open(&mut self, secret_name: &str) {
        self.original_secret_name = secret_name.to_string();
        self.updated_secret_name = secret_name.to_string();
        if secret_name.is_empty() {
            self.title = ADD_SECRET_TITLE.to_string();
            self.dialog_secrets = CREDENTIAL_FIELDS.iter().map(|&key| (key.to_string(), "".to_string())).collect();
        } else {
            self.title = format!("{}: {}", MODIFY_SECRET_TITLE, secret_name);
            self.dialog_secrets = self.load_secrets(secret_name);
        }

        self.open_dialog = true;
    }

    fn load_secrets(&mut self, secret_name: &str) -> Vec<(String, String)> {
        let mut secrets: Vec<(String, String)> = self
            .credentials_provider
            .load_secrets(secret_name)
            .expect(format!("cannot load secret {}", secret_name).as_str())
            .into_iter()
            .collect();

        secrets.sort_by_key(|(key, _)| (CREDENTIAL_FIELDS.iter().position(|&k| k == key).unwrap_or(CREDENTIAL_FIELDS.len()), key.clone()));
        secrets
    }

    fn close(&mut self) {
        self.open_dialog = false;
        self.original_secret_name = "".to_string();
        self.error_message = None;
    }

    fn show_editable_section(&mut self, ui: &mut Ui) {
        let mut potential_index: Option<usize> = Option::None;

        self.dialog_secrets.iter_mut().enumerate().for_each(|(index, (key, value))| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(key);
                ui.text_edit_singleline(value);
                if ui.button(DELETE_BUTTON_LABEL).clicked() {
                    potential_index = Some(index);
                }
            });
        });

        if let Some(index_to_remove) = potential_index {
            self.dialog_secrets.remove(index_to_remove);
        }

        ui.add_space(6.0);
        if ui.button(ADD_BUTTON_LABEL).clicked() {
            let len = self.dialog_secrets.len() + 1;
            self.dialog_secrets.push((format!("name{len}"), format!("value{len}")));
        }
    }

    pub fn show(&mut self, ctx: &Context) {
        if self.open_dialog {
            let modify_secret_dialog = ViewportBuilder::default().with_title(self.title.as_str()).with_close_button(true).with_decorations(true);
            let dialog_id = ViewportId::from_hash_of("modify_secret_dialog");

            ctx.show_viewport_immediate(dialog_id, modify_secret_dialog, |ctx, _| {
                if ctx.input(|input_state| input_state.viewport().close_requested()) {
                    self.close();
                }
                TopBottomPanel::bottom(Id::new("modify_bottom_panel")).show(ctx, |ui| {
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label("Secret name: ");
                        ui.text_edit_singleline(&mut self.updated_secret_name);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.button(CANCEL_BUTTON_LABEL).clicked() {
                                self.close();
                            }
                            if ui.button(SAVE_BUTTON_LABEL).clicked() {
                                self.handle_save(ctx);
                            }
                        })
                    });
                    ui.add_space(2.0);
                });
                CentralPanel::default().show(ctx, |ui| {
                    self.show_editable_section(ui);

                    ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                        if let Some(error) = &self.error_message {
                            ui.colored_label(ui.style().visuals.error_fg_color, error);
                        }
                    });
                });
            });
        }
    }

    fn handle_save(&mut self, ctx: &Context) {
        if self.updated_secret_name.is_empty() {
            self.error_message = Some("Secret name cannot be empty.".to_string());
        } else {
            let secrets_to_save: std::collections::HashMap<String, String> = self.dialog_secrets.iter().cloned().collect();
            let original_secret_name = if self.original_secret_name.is_empty() { None } else { Some(self.original_secret_name.as_str()) };

            match self.credentials_provider.update_secret(original_secret_name, &self.updated_secret_name, &secrets_to_save) {
                Ok(_) => {
                    Self::clear_ui_cache(ctx, original_secret_name);
                    self.close();
                }
                Err(e) => {
                    self.error_message = Some(format!("Unable to save secret: {}", e));
                }
            }
        }
    }

    fn clear_ui_cache(ctx: &Context, original_secret_name: Option<&str>) {
        let secret_names_cache_id = Id::new("secret_names").with("cache");
        ctx.memory_mut(|m| m.data.remove::<Vec<String>>(secret_names_cache_id));

        if let Some(name) = original_secret_name {
            let secret_cache_id = Id::new(name).with("cache");
            ctx.memory_mut(|m| m.data.remove::<Vec<(String, String)>>(secret_cache_id));
        }
    }
}
