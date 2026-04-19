mod secret_section;

use std::rc::Rc;

use crate::credentials_provider::CredentialsProvider;
use crate::delete_secret::DeleteSecretUI;
use crate::modify_secret::ModifySecretUI;
use crate::view_secret::secret_section::SecretSectionUI;
use eframe::{App, Frame};
use eframe::egui::{
    Align, CentralPanel, FontFamily, FontId, Id, Key, Layout, Panel, TextEdit,
    TextStyle, Ui, Vec2, ViewportBuilder, ViewportCommand,
};

pub struct ViewSecretUI {
    credentials_provider: Rc<CredentialsProvider>,
    search_field: Id,
    search_term: String,
    initial_search_focus: bool,
    modify_secret_ui: ModifySecretUI,
    delete_secret_ui: DeleteSecretUI,
    secret_section_ui: SecretSectionUI,
}

impl ViewSecretUI {
    pub fn new(credentials_provider: &Rc<CredentialsProvider>, _version: String) -> Self {
        Self {
            credentials_provider: Rc::clone(credentials_provider),
            search_field: Id::new("search_field"),
            search_term: String::new(),
            initial_search_focus: false,
            modify_secret_ui: ModifySecretUI::new(credentials_provider),
            delete_secret_ui: DeleteSecretUI::new(credentials_provider),
            secret_section_ui: SecretSectionUI::new(credentials_provider),
        }
    }

    pub fn run(self, version: String) -> eframe::Result {
        let title = format!("Rustillium v.{}", version);
        let options = eframe::NativeOptions {
            viewport: ViewportBuilder::default().with_inner_size(Vec2::new(640.0, 480.0)),
            ..Default::default()
        };
        eframe::run_native(
            &title,
            options,
            Box::new(|_cc| Ok(Box::new(self))),
        )
    }

    fn focus_on_search(&mut self, ui: &Ui) {
        if !self.initial_search_focus {
            ui.memory_mut(|m| m.request_focus(self.search_field));
            self.initial_search_focus = true;
        }
    }

    fn apply_custom_styles(ui: &Ui) {
        ui.ctx().global_style_mut(|style| {
            style.text_styles.insert(TextStyle::Button, FontId::new(16.00, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Body, FontId::new(16.00, FontFamily::Proportional));
            style.text_styles.insert(TextStyle::Monospace, FontId::new(16.00, FontFamily::Monospace));
            style.spacing.button_padding = Vec2::new(4.0, 2.0);
        });
    }

    fn build_bottom_panel(&mut self, ui: &mut eframe::egui::Ui) {
        Panel::bottom(Id::new("bottom_panel")).show_inside(ui, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                     if ui.button("\u{2bab} Exit").clicked() {
                         ViewSecretUI::close(ui.ctx());
                    };
                    if ui.button("\u{2795} Add Secret").clicked() {
                        self.modify_secret_ui.open("");
                    }
                });
            });
            ui.add_space(2.0);
        });
    }

    fn close(ctx: &eframe::egui::Context) {
        ctx.send_viewport_cmd(ViewportCommand::Close);
    }

    fn load_secret_names(&self, ui: &Ui) -> Vec<String> {
        let cache_id = Id::new("secret_names").with("cache");
        let cached_secret_names: Option<Vec<String>> = ui.data(|reader| reader.get_temp(cache_id));

        
        if let Some(secret_names) = cached_secret_names {
            secret_names
        } else {
            let loaded_secret_names = self.credentials_provider.load_secret_names().expect("Cannot load secret names");
            ui.data_mut(|writer| {
                writer.insert_temp(cache_id, loaded_secret_names.clone());
            });
            loaded_secret_names
        }
    }
}

impl App for ViewSecretUI {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut Frame) {
        Self::apply_custom_styles(ui);
        self.focus_on_search(ui);

        self.build_bottom_panel(ui);

        let secrets = self.load_secret_names(ui);
        let search_term = &mut self.search_term;

        CentralPanel::default().show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search: ");
                ui.add_sized(
                    ui.available_size(),
                    TextEdit::singleline(search_term)
                        .id(self.search_field)
                        .hint_text("search by secret name"),
                );
            });
            eframe::egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                let search_lower = search_term.to_lowercase();
                secrets
                    .iter()
                    .filter(|secret| secret.to_lowercase().contains(&search_lower))
                    .for_each(|secret| {
                        self.secret_section_ui
                            .show(ui, secret, &mut self.modify_secret_ui, &mut self.delete_secret_ui);
                    });
            });
        });

        // Show modify/delete dialog viewports
        self.modify_secret_ui.show(ui);
        self.delete_secret_ui.show(ui);

        // Keyboard shortcuts
        if ui.input(|i| i.key_pressed(Key::F) && i.modifiers.ctrl) {
            ui.memory_mut(|m| m.request_focus(self.search_field));
        }
        if ui.input(|i| i.key_pressed(Key::Q) && i.modifiers.ctrl) {
            Self::close(ui.ctx());
        }

        self.secret_section_ui.handle_popup(ui.ctx());
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit(&mut self) {}
}
