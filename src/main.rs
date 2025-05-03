use gtk::glib::PropertyGet;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, Builder, Box, Label, Button, Expander, Grid, Clipboard
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
    main_content: Box,
    credentials: Value
}

impl AppUI {
    fn populate_content(self: &Self) {
        if let Some(table) = self.credentials.as_table() {
            for (section_name, section) in table {
                let section_single = Expander::builder().use_markup(true).label(&format!("<b>{}</b>", section_name)).expanded(false).build();

                let grid: Grid = self.build_grid(section);

                section_single.add(&grid);

                self.main_content.pack_start(&section_single, false, false, 5);
            }
        }
    }

    fn build_labels(self: &Self, grid: &Grid, name: &str, value: &str, index: i32) {
        let name_label = Label::builder().use_markup(true).label(&format!("<i>{}</i>", name)).margin_start(25).halign(gtk::Align::Start).build();
        grid.attach(&name_label, 0, index, 1, 1);

        let value_button = Button::builder().label(value).halign(gtk::Align::Start).build();
        value_button.connect_clicked(|btn| {
            let primary_clipboard = Clipboard::get(&gtk::gdk::SELECTION_PRIMARY);
            let selection_clipboard = Clipboard::get(&gtk::gdk::SELECTION_CLIPBOARD);
            primary_clipboard.set_text(btn.label().unwrap().as_str());
            selection_clipboard.set_text(btn.label().unwrap().as_str());
        });

        grid.attach(&value_button, 1, index, 1, 1);

    }

    fn build_grid(self: &Self, section: &Value) -> Grid {
        let grid = Grid::new();
        grid.set_column_spacing(25);
        grid.set_row_spacing(5);

        if let Some(section_table) = section.as_table() {
            let mut addon = 0;
            if let Some(login) = section_table["login"].as_str() {
                self.build_labels(&grid, "login", login, 0);
                addon = addon+1;
            }

            if let Some(password) = section_table["password"].as_str() {
                self.build_labels(&grid, "password", password, 1);
                addon = addon+1;
            }


             for (index, (key, value)) in section_table.iter().enumerate() {
                if key == "login" || key == "password" {
                    continue;
                }
                self.build_labels(&grid, key, value.as_str().unwrap(), (index as i32)+addon);
            }
        };
        grid
    }

}

fn build_ui(application: &gtk::Application) {

    let builder = Builder::from_file("ui/rustillum.ui");

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let credentials_as_string = fs::read_to_string("data/cre.toml").expect("Couldn't read cre.toml");
    let credentials: Value = toml::from_str(&credentials_as_string).expect("Couldn't parse cre.toml");

    let main_content: Box = builder.object("main_box").expect("Couldn't get tree_view");
    main_content.style_context().add_class("large-font");
    let provider = gtk::CssProvider::new();
    provider.load_from_data(b".large-font { font-size: 12pt }").expect("Can't load from data");
    gtk::StyleContext::add_provider_for_screen(
        &gtk::gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );


    let ui = AppUI {
        main_content,
        credentials
    };

    ui.populate_content();

    window.show_all();
}


