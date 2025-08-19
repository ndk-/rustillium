use std::rc::Rc;

use crate::credentials_provider::CredentialsProvider;
use eframe::egui::{Align, CentralPanel, Context, Id, Layout, TextEdit, TopBottomPanel, Vec2, ViewportBuilder, ViewportId};

const DELETE_SECRET_TITLE: &str = "Delete Secret";
const CANCEL_BUTTON_LABEL: &str = "\u{274c} Cancel";
const DELETE_BUTTON_LABEL: &str = "\u{1f5d1} Delete";

pub struct DeleteSecretUI {
    credentials_provider: Rc<CredentialsProvider>,
    secret_name: String,
    confirmation_text: String,
    open_dialog: bool,
    error_message: Option<String>,
}

impl DeleteSecretUI {
    pub fn new(credentials_provider: &Rc<CredentialsProvider>) -> Self {
        Self {
            credentials_provider: Rc::clone(credentials_provider),
            secret_name: "".to_string(),
            confirmation_text: "".to_string(),
            open_dialog: false,
            error_message: None,
        }
    }

    pub fn open(&mut self, secret_name: &str) {
        self.secret_name = secret_name.to_string();
        self.confirmation_text = "".to_string();
        self.error_message = None;
        self.open_dialog = true;
    }

    fn close(&mut self) {
        self.open_dialog = false;
        self.secret_name = "".to_string();
        self.confirmation_text = "".to_string();
        self.error_message = None;
    }

    pub fn show(&mut self, ctx: &Context) {
        if self.open_dialog {
            let delete_secret_dialog = ViewportBuilder::default()
                .with_inner_size(Vec2::new(450.0, 125.0))
                .with_title(format!("{}: {}", DELETE_SECRET_TITLE, self.secret_name))
                .with_close_button(true)
                .with_decorations(true);
            let dialog_id = ViewportId::from_hash_of("delete_secret_dialog");

            ctx.show_viewport_immediate(dialog_id, delete_secret_dialog, |ctx, _| {
                if ctx.input(|input_state| input_state.viewport().close_requested()) {
                    self.close();
                }

                TopBottomPanel::bottom(Id::new("delete_bottom_panel")).show(ctx, |ui| {
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.button(CANCEL_BUTTON_LABEL).clicked() {
                                self.close();
                            }
                            if ui.button(DELETE_BUTTON_LABEL).clicked() {
                                self.handle_delete(ctx);
                            }
                        });
                    });
                    ui.add_space(2.0);
                });

                CentralPanel::default().show(ctx, |ui| {
                    ui.label("To confirm deletion, please type the secret name below:");
                    ui.add(TextEdit::singleline(&mut self.confirmation_text));
                    if let Some(error) = &self.error_message {
                        ui.colored_label(ui.style().visuals.error_fg_color, error);
                    }
                });
            });
        }
    }

    fn handle_delete(&mut self, ctx: &Context) {
        if self.confirmation_text != self.secret_name {
            self.error_message = Some("The entered name does not match the secret name.".to_string());
            return;
        }

        match self.credentials_provider.delete_secret(&self.secret_name) {
            Ok(_) => {
                Self::clear_ui_cache(ctx, &self.secret_name);
                self.close();
            }
            Err(e) => {
                self.error_message = Some(format!("Unable to delete secret: {}", e));
            }
        }
    }

    fn clear_ui_cache(ctx: &Context, secret_name: &str) {
        let secret_names_cache_id = Id::new("secret_names").with("cache");
        ctx.memory_mut(|m| m.data.remove::<Vec<String>>(secret_names_cache_id));

        let secret_cache_id = Id::new(secret_name).with("cache");
        ctx.memory_mut(|m| m.data.remove::<Vec<(String, String)>>(secret_cache_id));
    }
}
