use gtk::prelude::*;
use gtk::{
    ApplicationWindow, TreeStore, TreeView, Builder, TreeViewColumn, CellRendererText
};
use std::fs;
use toml::Value;

fn build_ui(application: &gtk::Application) {
    let builder = Builder::from_file("ui/rustillum.glade");

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let section_view: TreeView = builder.object("tree_view").expect("Couldn't get tree_view");
    let section_view_store = TreeStore::new(&[String::static_type()]);

    let section_column = TreeViewColumn::new();
    let section_text = CellRendererText::new();

    gtk::prelude::TreeViewColumnExt::pack_start(&section_column, &section_text, true);
    gtk::prelude::TreeViewColumnExt::add_attribute(&section_column, &section_text, "text", 0);
    section_view.append_column(&section_column);

    section_view.set_model(Some(&section_view_store));
    section_view.set_headers_visible(false);

    let toml_string = fs::read_to_string("data/cre.toml").expect("Couldn't read cre.toml");
    let toml_value: Value = toml::from_str(&toml_string).expect("Couldn't parse cre.toml");

    if let Some(table) = toml_value.as_table() {
        for (section_name, _) in table {
            let iter = section_view_store.append(None);
            section_view_store.set_value(&iter, 0, &section_name.to_value());
        }
    }

    window.show_all();
}

const APP_ID: &str = "org.rustillum.ui";

fn main() {
    // Create a new application
    let app = gtk::Application::new(
        Some(APP_ID),
        Default::default(),
    );

    // Connect to "activate" signal of application
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
