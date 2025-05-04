use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};
use std::rc::Rc;

pub mod gtk_ui;
pub mod credentials_provider;

use crate::credentials_provider::CredentialsProvider;
use crate::gtk_ui::GtkUI;

const APP_ID: &str = "org.rustillum.ui";

fn main() {
    // Create a new application
    let app = gtk::Application::new(Some(APP_ID), Default::default());

    // Connect to "activate" signal of application
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &gtk::Application) {
    let builder = Builder::from_file("ui/rustillum.ui");

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let credentials_provider = Rc::new(CredentialsProvider::new("./secrets"));

    let ui = GtkUI::new(builder, credentials_provider);

    ui.build();

    window.show_all();
}

