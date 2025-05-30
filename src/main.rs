use config::Config;
use gtk::prelude::*;
use std::rc::Rc;

pub mod credentials_provider;
pub mod gtk_ui;

use crate::credentials_provider::CredentialsProvider;
use crate::gtk_ui::GtkUI;
use std::env::var as environment_variable;

const APP_ID: &str = "org.rustillum.ui";

fn main() {
    let app = gtk::Application::new(Some(APP_ID), Default::default());

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(application: &gtk::Application) {
    let credentials_provider = configure_credentials_provider();

    let ui = GtkUI::new(credentials_provider);

    ui.show(application);
}

fn configure_credentials_provider() -> Rc<CredentialsProvider> {
    let mut config_path = environment_variable("HOME").unwrap_or(".".to_string());
    config_path.push_str("/.config/rustillium/config.toml");
    
    let config = Config::builder()
        .add_source(config::File::with_name(config_path.as_str()).required(false))
        .add_source(config::Environment::with_prefix("RUSTILLIUM"))
        .build()
        .expect("Cannot read configuration");
    
    let secrets_directory = config
        .get_string("secrets_directory")
        .unwrap_or("./enc".to_string());
    
    return Rc::new(CredentialsProvider::new(&secrets_directory));
}
