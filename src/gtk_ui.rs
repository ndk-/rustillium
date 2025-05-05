use gtk::prelude::*;
use gtk::{Box, Button, Clipboard, Expander, Grid, Label, Builder, ApplicationWindow, SearchBar, SearchEntry};
use gtk::glib::Object;
use std::{collections::HashMap, rc::Rc};

use crate::credentials_provider::CredentialsProvider;

pub struct GtkUI {
    builder: Rc<Builder>,
    credentials_provider: Rc<CredentialsProvider>,
}

impl GtkUI {    
    pub fn new(credentials_provider: Rc<CredentialsProvider>) -> Self {
        let builder = Rc::from(Builder::from_file("ui/rustillum.ui"));
        Self { builder, credentials_provider}
    }

    pub fn show(self: &Self, application: &gtk::Application) {
        let window = self.get_window();
        window.set_application(Some(application));

        self.style_main_component();
        self.build_search_component();
        self.build_secrets_component();

        window.show_all();
    }

    fn build_search_component(self: &Self) {
        let search_bar: SearchBar = self.get_component("search_bar");
        search_bar.set_search_mode(true);

        let search_entry: SearchEntry = self.get_component("search_entry");

        let builder = Rc::clone(&self.builder);
        search_entry.connect_changed(GtkUI::on_search_filter_results(builder));
    }

    fn on_search_filter_results(builder: Rc<Builder>) -> impl Fn(&SearchEntry) {
        move |entry| {
            let parent: Box = builder.object("main_box").expect("Can't find main component");
            for child in parent.children() {
                let visibility = child.widget_name().contains(entry.text().as_str());
                child.set_visible(visibility); 
            }
        }
    }

    fn get_component<T: IsA<Object>>(self: &Self, component_name: &str) -> T {
        return self.builder.object(component_name).expect(format!("Couldn't get component: {}", component_name).as_str());
    }

    fn get_window(self: &Self) -> ApplicationWindow {
        return self.get_component("window");
    }

    fn get_main_component(self: &Self) -> Box {
        return self.get_component("main_box");
    }

    fn style_main_component(self: &Self) {
        self.get_main_component().style_context().add_class("large-font");
        let provider = gtk::CssProvider::new();
    
        provider
            .load_from_data(b".large-font { font-size: 12pt }")
            .expect("Can't load from data");
    
        gtk::StyleContext::add_provider_for_screen(
            &gtk::gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn build_secrets_component(self: &Self) {
        let secret_names = self.credentials_provider.load_secret_names().expect("unable to load secret names");
        let main_component = self.get_main_component();
        for section_name in secret_names {
            let secret_section = Expander::builder()
                .name(&section_name)
                .use_markup(true)
                .label(&format!("<b>{}</b>", &section_name))
                .expanded(false)
                .build();

            secret_section.connect_expanded_notify(
                GtkUI::on_expanded_populate_secrets(
                    Rc::clone(&self.credentials_provider)
                )
            );

            main_component.pack_start(&secret_section, false, false, 5);
        }
    }

    fn on_expanded_populate_secrets(credentials_provider: Rc<CredentialsProvider>) -> impl Fn(&Expander) {
        move |expander| {
            let secret_name = expander.widget_name();
            let secrets = credentials_provider.load_secrets(secret_name.as_str()).expect(format!("Can't load a secret {}", secret_name.as_str()).as_str());

            let grid = GtkUI::build_single_secret_section(&secrets);
            grid.show_all();

            if let Some(child) = expander.child() {
                expander.remove(&child);
            }

            expander.add(&grid);
        }
    }
    
    fn build_single_secret_section(section: &HashMap<String, String>) -> Grid {
        let grid = Grid::builder().column_spacing(25).row_spacing(5).build();

            let mut index_offset = 0;
            if let Some(login) = section.get("login") {
                GtkUI::build_single_secret(&grid, "login", login, 0);
                index_offset = index_offset + 1;
            }

            if let Some(password) = section.get("password") {
                GtkUI::build_single_secret(&grid, "password", password, 1);
                index_offset = index_offset + 1;
            }

            for (index, (key, value)) in section.iter().enumerate() {
                if key == "login" || key == "password" {
                    continue;
                }
                GtkUI::build_single_secret(&grid, key, value, (index as i32) + index_offset);
            }
        grid
    }

    fn build_single_secret(grid: &Grid, name: &str, value: &str, index: i32) {
        let name_label = Label::builder()
            .use_markup(true)
            .label(&format!("<i>{}</i>", name))
            .margin_start(25)
            .halign(gtk::Align::Start)
            .build();

        grid.attach(&name_label, 0, index, 1, 1);

        let value_button = Button::builder()
            .label(value)
            .halign(gtk::Align::Start)
            .build();

        value_button.connect_clicked(|button| {
            let primary_clipboard = Clipboard::get(&gtk::gdk::SELECTION_PRIMARY);
            let selection_clipboard = Clipboard::get(&gtk::gdk::SELECTION_CLIPBOARD);
            primary_clipboard.set_text(button.label().unwrap().as_str());
            selection_clipboard.set_text(button.label().unwrap().as_str());
        });

        grid.attach(&value_button, 1, index, 1, 1);
    }
}
