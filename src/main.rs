use config::Config;
use std::{env::var as environment_variable, rc::Rc};

pub mod credentials_provider;
pub mod delete_secret;
pub mod modify_secret;
pub mod view_secret;

use crate::{credentials_provider::CredentialsProvider, view_secret::ViewSecretUI};

fn main() -> eframe::Result {
    let credentials_provider = configure_credential_provider();

    let view_secret_ui = ViewSecretUI::new(&Rc::new(credentials_provider));

    return view_secret_ui.show();
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
    let recipient_email = config.get_string("recipient_email").expect("recipient_email is not set in the configuration");

    return CredentialsProvider::new(&secrets_directory, &recipient_email);
}
