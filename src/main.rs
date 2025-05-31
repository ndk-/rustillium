use config::Config;
use std::env::var as environment_variable;

pub mod credentials_provider;
pub mod egui_ui;

use crate::credentials_provider::CredentialsProvider;
use crate::egui_ui::AppUI;

fn main() -> eframe::Result {
    let credentials_provider = configure_credential_provider();

    let ui = AppUI::new(credentials_provider);

    return ui.show();
}

fn configure_credential_provider() -> CredentialsProvider {
    let mut config_path = environment_variable("HOME").unwrap_or(".".to_string());
    config_path.push_str("/.config/rustillium/config.toml");
    
    let config = Config::builder()
        .add_source(config::File::with_name(&config_path).required(false))
        .add_source(config::Environment::with_prefix("RUSTILLIUM"))
        .build()
        .expect("Cannot read configuration");

    let secrets_directory = config.get_string("secrets_directory").unwrap_or("./enc".to_string());

    return CredentialsProvider::new(&secrets_directory);
}
