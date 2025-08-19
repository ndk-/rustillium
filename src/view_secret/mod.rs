mod secret_section;

use std::rc::Rc;

use crate::credentials_provider::CredentialsProvider;
use crate::delete_secret::DeleteSecretUI;
use crate::modify_secret::ModifySecretUI;
use crate::view_secret::secret_section::SecretSectionUI;
use eframe::egui::{Align, CentralPanel, Context, FontFamily, FontId, Id, Key, Layout, ScrollArea, TextEdit, TextStyle, TopBottomPanel, Ui, ViewportBuilder, ViewportCommand, vec2};

pub struct ViewSecretUI {
    credentials_provider: Rc<CredentialsProvider>,
    search_field: Id,
    initial_search_focus: bool,
    modify_secret_ui: ModifySecretUI,
    delete_secret_ui: DeleteSecretUI,
    secret_section_ui: SecretSectionUI,
}

impl ViewSecretUI {
    pub fn new(credentials_provider: &Rc<CredentialsProvider>) -> Self {
        Self {
            credentials_provider: Rc::clone(credentials_provider),
            search_field: Id::new("search_field"),
            initial_search_focus: false,
            modify_secret_ui: ModifySecretUI::new(credentials_provider),
            delete_secret_ui: DeleteSecretUI::new(credentials_provider),
            secret_section_ui: SecretSectionUI::new(credentials_provider),
        }
    }

    pub fn show(mut self) -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: ViewportBuilder::default().with_inner_size([640.0, 480.0]),
            ..Default::default()
        };

        let mut search_term = "".to_owned();

        eframe::run_simple_native("Rustillium", options, move |ctx, _frame| {
            let secrets = self.load_secret_names(ctx);

            self.create_keyboard_shortcuts(ctx);
            self.focus_on_search(ctx);

            ViewSecretUI::apply_custom_styles(ctx);

            CentralPanel::default().show(ctx, |ui| {
                self.build_search_field(ui, &mut search_term);
                self.build_secrets_section(ui, &search_term, &secrets);
            });

            self.build_bottom_panel(ctx);
            self.secret_section_ui.handle_popup(ctx);
            self.modify_secret_ui.show(ctx);
            self.delete_secret_ui.show(ctx);
        })
    }

    fn focus_on_search(&mut self, ctx: &Context) {
        if !self.initial_search_focus {
            ctx.memory_mut(|m| m.request_focus(self.search_field));
            self.initial_search_focus = true;
        }
    }

    fn apply_custom_styles(ctx: &Context) {
        ctx.style_mut(|style| {
            style.text_styles.insert(TextStyle::Button, FontId::new(16.00, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Body, FontId::new(16.00, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Monospace, FontId::new(16.00, FontFamily::Monospace));
            style.spacing.button_padding = vec2(4.0, 2.0);
        });
    }

    fn build_bottom_panel(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("bottom_panel").show_separator_line(true).show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui.button("\u{2bab} Exit").clicked() {
                        ViewSecretUI::close(ctx);
                    };
                    if ui.button("\u{2795} Add Secret").clicked() {
                        self.modify_secret_ui.open("");
                    }
                });
                ui.add_space(2.0);
            });
        });
    }

    fn create_keyboard_shortcuts(&mut self, ctx: &Context) {
        if ctx.input(|i| i.key_pressed(Key::F) && i.modifiers.ctrl) {
            ctx.memory_mut(|m| m.request_focus(self.search_field));
        }

        if ctx.input(|i| i.key_pressed(Key::Q) && i.modifiers.ctrl) {
            ViewSecretUI::close(ctx);
        }
    }

    fn close(ctx: &Context) {
        ctx.send_viewport_cmd(ViewportCommand::Close);
    }

    fn build_secrets_section(&mut self, ui: &mut Ui, search_term: &str, secrets: &Vec<String>) {
        ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
            secrets.into_iter().filter(|secret| secret.contains(search_term)).for_each(|secret| {
                self.secret_section_ui.show(ui, &secret, &mut self.modify_secret_ui, &mut self.delete_secret_ui);
            });
        });
    }

    fn build_search_field(&self, ui: &mut Ui, search_term: &mut String) {
        ui.horizontal(|ui| {
            ui.label("Search: ");
            ui.add_sized(ui.available_size(), TextEdit::singleline(search_term).id(self.search_field).hint_text("search by secret name"));
        });
    }

    fn load_secret_names(&self, ctx: &Context) -> Vec<String> {
        let cache_id = Id::new("secret_names").with("cache");
        let cached_secret_names: Option<Vec<String>> = ctx.data(|reader| reader.get_temp(cache_id));

        let secret_names_to_display = if let Some(secret_names) = cached_secret_names {
            secret_names
        } else {
            let loaded_secret_names = self.credentials_provider.load_secret_names().expect("Cannot load secret names");
            ctx.data_mut(|writer| {
                writer.insert_temp(cache_id, loaded_secret_names.clone());
            });
            loaded_secret_names
        };
        secret_names_to_display
    }
}
