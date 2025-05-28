pub mod credentials_provider;
pub mod egui_ui;

use crate::credentials_provider::CredentialsProvider;
use crate::egui_ui::AppUI;

fn main() -> eframe::Result {
    let credentials_provider = CredentialsProvider::new("./enc".to_string());

    let ui = AppUI::new(credentials_provider);

    return ui.show();
}
