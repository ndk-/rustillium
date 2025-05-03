use gtk::prelude::*;
use gtk::{ApplicationWindow, Box, Builder};
use std::fs;
use toml::Value;

const APP_ID: &str = "org.rustillum.ui";

mod app_ui;

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

    let credentials_as_string = fs::read_to_string("data/cre.toml").expect("Couldn't read cre.toml");
    let credentials: Value = toml::from_str(&credentials_as_string).expect("Couldn't parse cre.toml");

    let main_content = build_main_component(builder);
    let ui = app_ui::AppUI::new(main_content, credentials);

    ui.populate_content();

    window.show_all();
}

fn build_main_component(builder: Builder) -> Box {
    let main_content: Box = builder.object("main_box").expect("Couldn't get tree_view");

    main_content.style_context().add_class("large-font");
    let provider = gtk::CssProvider::new();

    provider
        .load_from_data(b".large-font { font-size: 12pt }")
        .expect("Can't load from data");

    gtk::StyleContext::add_provider_for_screen(
        &gtk::gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    main_content
}
