use crate::credentials_provider::CredentialsProvider;
use eframe::egui::{self, Align, Button, CentralPanel, Id, Key, Layout, PopupCloseBehavior, ScrollArea, TextEdit, TopBottomPanel, ViewportBuilder, Widget};
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct PopupState {
    id: Id,
    opened_at: Instant,
}

pub struct AppUI {
    credentials_provider: CredentialsProvider,
    search_field: Id,
    popup_state: Option<PopupState>,
}

impl AppUI {
    pub fn new(credentials_provider: CredentialsProvider) -> Self {
        Self {
            credentials_provider,
            search_field: Id::new("search_field"),
            popup_state: None,
        }
    }

    pub fn show(mut self) -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: ViewportBuilder::default().with_inner_size([640.0, 480.0]),
            ..Default::default()
        };

        let mut search_term = "".to_owned();
        let secrets = self.credentials_provider.load_secret_names().expect("Failed to load secret names");

        eframe::run_simple_native("Rustillium", options, move |ctx, _frame| {
            self.create_keyboard_shortcuts(ctx);
            
            AppUI::apply_custom_styles(ctx);

            CentralPanel::default().show(ctx, |ui| {
                self.build_search_field(ui, &mut search_term);
                self.build_secrets_section(ui, &search_term, &secrets);
            });

            AppUI::build_bottom_panel(ctx);

            self.handle_popup(ctx);
        })
    }

    fn apply_custom_styles(ctx: &egui::Context) {
        ctx.style_mut(|style| {
            style.text_styles.insert(egui::TextStyle::Button, egui::FontId::new(16.00, egui::FontFamily::Proportional));
            style.text_styles.insert(egui::TextStyle::Body, egui::FontId::new(16.00, egui::FontFamily::Proportional));
            style.text_styles.insert(egui::TextStyle::Monospace, egui::FontId::new(16.00, egui::FontFamily::Monospace));
            style.spacing.button_padding = egui::vec2(4.0, 2.0);
        });
    }

    fn build_bottom_panel(ctx: &egui::Context) {
        TopBottomPanel::bottom("bottom_panel").show_separator_line(true).show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                ui.add_space(6.0);
                if ui.button("Exit").clicked() {
                    AppUI::close(ctx);
                };
                ui.add_space(2.0);
            });
        });
    }

    fn create_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(Key::F) && i.modifiers.ctrl) {
            ctx.memory_mut(|m| m.request_focus(self.search_field));
        }

        if ctx.input(|i| i.key_pressed(Key::Q) && i.modifiers.ctrl) {
            AppUI::close(ctx);
        }
    }

    fn close(ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    fn handle_popup(&mut self, ctx: &egui::Context) {
        if let Some(popup) = &self.popup_state {
            if popup.opened_at.elapsed() >= Duration::from_secs(1) {
                egui::Popup::close_id(ctx, popup.id);
                self.popup_state = None;
            }
            ctx.request_repaint_after(Duration::from_micros(500));
        }
    }

    fn build_secrets_section(&mut self, ui: &mut egui::Ui, search_term: &str, secrets: &Vec<String>) {
        ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
            secrets.into_iter().filter(|secret| secret.contains(search_term)).for_each(|secret| {
                self.build_secret_section(ui, &secret);
            });
        });
    }

    fn build_secret_section(&mut self, ui: &mut egui::Ui, secret: &str) {
        ui.collapsing(secret, |ui| {
            self.load_secrets(ui, secret).iter().for_each(|(key, value)| {

                ui.horizontal(|ui| {
                        ui.label(key);

                        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                            let popup_id = Id::new(key);
                            let button = Button::new(value).fill(ui.ctx().theme().default_visuals().extreme_bg_color).ui(ui);

                            if button.clicked() {
                                self.popup_state = Some(PopupState { id: popup_id, opened_at: Instant::now() });
                                ui.ctx().copy_text(value.clone());
                            }

                            egui::Popup::from_toggle_button_response(&button).close_behavior(PopupCloseBehavior::CloseOnClick).id(popup_id).show(|ui| {
                                ui.label("Secret has been copied!");
                            });
                        });
                });
            });
        });
    }

    fn load_secrets(&self, ui: &mut egui::Ui, secret: &str) -> Vec<(String, String)> {
        let cached_secrets: Option<Vec<(String, String)>> = ui.data(|reader| reader.get_temp(ui.id()));

        let secrets_to_display = if let Some(secrets) = cached_secrets {
            secrets
        } else {
            let loaded_secrets = AppUI::to_displayed_secrets(self.credentials_provider.load_secrets(secret).expect("Cannot load secrets file"));
            ui.data_mut(|writer| {
                writer.insert_temp(ui.id(), loaded_secrets.clone());
            });
            loaded_secrets
        };
        secrets_to_display
    }

    fn build_search_field(&self, ui: &mut egui::Ui, search_term: &mut String) {
        ui.horizontal(|ui| {
            ui.label("Search: ");
            ui.add_sized(ui.available_size(), TextEdit::singleline(search_term).id(self.search_field).hint_text("search by secret name"));
        });
    }

    fn to_displayed_secrets(mut secrets: HashMap<String, String>) -> Vec<(String, String)> {
        let mut result: Vec<(String, String)> = Vec::new();

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
