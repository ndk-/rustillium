use crate::credentials_provider::CredentialsProvider;
use crate::delete_secret::DeleteSecretUI;
use crate::modify_secret::ModifySecretUI;
use eframe::egui::{Align, Button, Context, Id, Layout, Popup, PopupCloseBehavior, RectAlign, Ui, Widget, collapsing_header};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};
use crate::totp_provider::{generate_totp_display_info};

pub struct SecretSectionUI {
    credentials_provider: Rc<CredentialsProvider>,
    popup_state: Option<PopupState>,
}

struct PopupState {
    id: Id,
    opened_at: Instant,
}

impl SecretSectionUI {
    pub fn new(credentials_provider: &Rc<CredentialsProvider>) -> Self {
        Self {
            credentials_provider: Rc::clone(credentials_provider),
            popup_state: None,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, secret: &str, modify_secret_ui: &mut ModifySecretUI, delete_secret_ui: &mut DeleteSecretUI) {
        let collapsible_state = collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), Id::new(secret), false);
        let is_collapsible_open = collapsible_state.is_open();
        collapsible_state
            .show_header(ui, |ui| {
                Self::build_header(secret, is_collapsible_open, modify_secret_ui, delete_secret_ui, ui);
            })
            .body(|ui| {
                self.load_secrets(ui, secret).iter().for_each(|(key, value)| {
                    self.build_secret_section(key, value, ui);
                });
            });
    }

    fn build_secret_section(&mut self, key: &String, value: &String, ui: &mut Ui) {
        if key == "totpurl" {
            self.build_totp_section(key, value, ui);
        } else {
            self.build_single_secret_section(key, value, ui);
        }
    }

    fn build_totp_section(&mut self, key: &String, value: &String, ui: &mut Ui) {
        let totp = generate_totp_display_info(&value).expect("Cannot parse totp code");
        ui.horizontal(|ui| {
            ui.label("TOTP code");
            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                let popup_id = Id::new(key);
                let totp_code_as_button = Button::new(&totp.code).fill(ui.ctx().theme().default_visuals().faint_bg_color).ui(ui);

                if totp_code_as_button.clicked() {
                    self.copy_secret(&totp.code, popup_id, ui);
                }

                ui.ctx().request_repaint_after(Duration::from_secs(1));

                Popup::from_toggle_button_response(&totp_code_as_button).close_behavior(PopupCloseBehavior::CloseOnClick).id(popup_id).show(|ui| {
                    ui.label("TOTP code has been copied!");
                });
                ui.label(format!("{} seconds left", totp.remaining_seconds));
            });
            
        });
    }


    fn build_single_secret_section(&mut self, key: &String, value: &String, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(key);
            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                let popup_id = Id::new(key);
                let secret_value_as_button = Button::new(value).fill(ui.ctx().theme().default_visuals().faint_bg_color).ui(ui);

                if secret_value_as_button.clicked() {
                    self.copy_secret(value, popup_id, ui);
                }

                Popup::from_toggle_button_response(&secret_value_as_button).close_behavior(PopupCloseBehavior::CloseOnClick).id(popup_id).show(|ui| {
                    ui.label("Secret has been copied!");
                });
            });
        });
    }

    fn copy_secret(&mut self, value: &String, popup_id: Id, ui: &mut Ui) {
        self.popup_state = Some(PopupState {
            id: popup_id,
            opened_at: Instant::now(),
        });
        ui.ctx().copy_text(value.clone());
    }

    fn build_header(secret: &str, is_collapsible_open: bool, modify_secret_ui: &mut ModifySecretUI, delete_secret_ui: &mut DeleteSecretUI, ui: &mut Ui) {
        ui.label(secret);
        if is_collapsible_open {
            let preferences_button = ui.button("⛭");
            Popup::menu(&preferences_button).align(RectAlign::RIGHT_START).show(|ui| {
                if ui.button("\u{1f58a} Modify").clicked() {
                    modify_secret_ui.open(secret);
                };
                if ui.button("\u{1f5d1} Delete").clicked() {
                    delete_secret_ui.open(secret);
                };
            });
        }
    }

    pub fn handle_popup(&mut self, ctx: &Context) {
        if let Some(popup) = &self.popup_state {
            if popup.opened_at.elapsed() >= Duration::from_secs(1) {
                Popup::close_id(ctx, popup.id);
                self.popup_state = None;
            }
            ctx.request_repaint_after(Duration::from_micros(500));
        }
    }

    fn load_secrets(&self, ui: &mut Ui, secret: &str) -> Vec<(String, String)> {
        let cache_id = Id::new(secret).with("cache");
        let cached_secrets: Option<Vec<(String, String)>> = ui.data(|reader| reader.get_temp(cache_id));

        let secrets_to_display = if let Some(secrets) = cached_secrets {
            secrets
        } else {
            let loaded_secrets = Self::to_displayed_secrets(self.credentials_provider.load_secrets(secret).expect("Cannot load secrets file"));
            ui.data_mut(|writer| {
                writer.insert_temp(cache_id, loaded_secrets.clone());
            });
            loaded_secrets
        };
        secrets_to_display
    }

    fn to_displayed_secrets(mut secrets: HashMap<String, String>) -> Vec<(String, String)> {
        let mut result: Vec<(String, String)> = Vec::new();

        if let Some(totpurl) = secrets.remove("totpurl") {
            result.push(("totpurl".to_string(), totpurl));
        }
        if let Some(username) = secrets.remove("username") {
            result.push(("username".to_string(), username));
        }
        if let Some(password) = secrets.remove("password") {
            result.push(("password".to_string(), password));
        }

        let mut remaining_secrets: Vec<(String, String)> = secrets.into_iter().collect();
        remaining_secrets.sort_by(|first, second| first.0.cmp(&second.0));

        result.extend(remaining_secrets);
        result
    }
}
