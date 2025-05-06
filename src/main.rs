use gtk::prelude::*;
use std::rc::Rc;

pub mod gtk_ui;
pub mod credentials_provider;

use crate::credentials_provider::CredentialsProvider;
use crate::gtk_ui::GtkUI;

const APP_ID: &str = "org.rustillum.ui";

fn main() {
    let app = gtk::Application::new(Some(APP_ID), Default::default());

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(application: &gtk::Application) {
    let credentials_provider = Rc::new(CredentialsProvider::new("./enc"));

    let ui = GtkUI::new(credentials_provider);

    ui.show(application);
}