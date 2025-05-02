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

    let tree_view: TreeView = builder.object("tree_view").expect("Couldn't get tree_view");
    let tree_store = TreeStore::new(&[String::static_type()]);

    let tree_view_column = TreeViewColumn::new();
    let cell_renderer_text = CellRendererText::new();

    gtk::prelude::TreeViewColumnExt::pack_start(&tree_view_column, &cell_renderer_text, true);
    gtk::prelude::TreeViewColumnExt::add_attribute(&tree_view_column, &cell_renderer_text, "text", 0);
    tree_view.append_column(&tree_view_column);

    tree_view.set_model(Some(&tree_store));
    tree_view.set_headers_visible(false);

    let toml_string = fs::read_to_string("data/cre.toml").expect("Couldn't read cre.toml");
    let toml_value: Value = toml::from_str(&toml_string).expect("Couldn't parse cre.toml");

    if let Some(table) = toml_value.as_table() {
        for (section_name, _) in table {
            let iter = tree_store.append(None);
            tree_store.set_value(&iter, 0, &section_name.to_value());
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
