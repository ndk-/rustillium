use gtk::prelude::*;
use gtk::{
    ApplicationWindow, TreeStore, TreeView, Builder, TreeViewColumn, CellRendererText
};
use std::fs;
use toml::Value;

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


struct AppUI {
    tree_view: TreeView,
    tree_store: TreeStore,
    credentials: Value
}

impl AppUI {
    fn create_tree_column(self: &Self, column_number: i32) {
        let column = TreeViewColumn::new();
        let text = CellRendererText::new();
        
        TreeViewColumnExt::pack_start(&column, &text, true);
        TreeViewColumnExt::add_attribute(&column, &text, "text", column_number);
        self.tree_view.append_column(&column);
    }

    fn populate_section(self: &Self) {
        if let Some(table) = self.credentials.as_table() {
            for (section_name, section) in table {
                let iter = self.tree_store.append(None);
                self.tree_store.set_value(&iter, 0, &section_name.to_value());
                
                self.populate_credentials(section, iter);
            }
        }
    }

    fn populate_credentials(self: &Self, section: &Value, iter: gtk::TreeIter) {

        if let Some(section_table) = section.as_table() {
            if let Some(login) = section_table["login"].as_str() {
                self.populate_field(iter, "login", login);
            }

            if let Some(password) = section_table["password"].as_str() {
                self.populate_field(iter, "password", password);
            }


             for (key, value) in section_table {
                if key == "login" || key == "password" {
                    continue;
                }
                self.populate_field(iter, key, value.as_str().unwrap());
            }
        };
    }

    fn populate_field(self: &Self, iter: gtk::TreeIter, name: &str, value: &str) {
        let child_iter: gtk::TreeIter = self.tree_store.append(Some(&iter));
        self.tree_store.set_value(&child_iter, 1, &name.to_value());
        self.tree_store.set_value(&child_iter, 2, &value.to_value());
    }
    
    fn build_ui(self: &Self) {
        self.create_tree_column(0);
        self.create_tree_column(1);
        self.create_tree_column(2);

        self.tree_view.set_model(Some(&self.tree_store));
        self.tree_view.set_headers_visible(false);

        self.populate_section();
    }


}

fn build_ui(application: &gtk::Application) {

    let builder = Builder::from_file("ui/rustillum.glade");

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let credentials_as_string = fs::read_to_string("data/cre.toml").expect("Couldn't read cre.toml");
    let credentials: Value = toml::from_str(&credentials_as_string).expect("Couldn't parse cre.toml");

    let ui = AppUI {
        tree_view: builder.object("tree_view").expect("Couldn't get tree_view"),
        tree_store: TreeStore::new(&[String::static_type(), String::static_type(), String::static_type()]),
        credentials
    };

    ui.build_ui();

    window.show_all();
}


