use gtk::{traits::{BinExt, BoxExt, ButtonExt, ContainerExt, ExpanderExt, GridExt, WidgetExt}, Box, Button, Clipboard, Expander, Grid, Label};
use std::{collections::HashMap, rc::Rc};

use crate::credentials_provider::CredentialsProvider;

pub struct AppUI {
    main_content: Box,
    credentials_provider: Rc<CredentialsProvider>,
}

impl AppUI {
    pub fn new(main_content: Box, credentials: Rc<CredentialsProvider>) -> Self {
        Self { main_content, credentials_provider: credentials }
    }

    pub fn populate_content(self: &Self) {
        self.populate_secret_names();
    }

    fn populate_secret_names(self: &Self) {
        let secret_names = self.credentials_provider.load_secret_names().expect("unable to load secret names");
        for section_name in secret_names {
            let secret_section = Expander::builder()
                .use_markup(true)
                .name(&section_name)
                .label(&format!("<b>{}</b>", &section_name))
                .expanded(false)
                .build();

            secret_section.connect_expanded_notify(
                AppUI::populate_secrets(
                    Rc::clone(&self.credentials_provider)
                )
            );

            self.main_content.pack_start(&secret_section, false, false, 5);
        }
    }

    fn populate_secrets(credentials_provider: Rc<CredentialsProvider>) -> impl Fn(&Expander) {
        move |expander| {
            let secret_name = expander.widget_name();
            let secrets = credentials_provider.load_secrets(secret_name.as_str()).expect(format!("Can't load a secret {}", secret_name.as_str()).as_str());

            let grid = AppUI::build_grid(&secrets);
            grid.show_all();

            if let Some(child) = expander.child() {
                expander.remove(&child);
            }

            expander.add(&grid);
        }
    }
    
    fn build_row(grid: &Grid, name: &str, value: &str, index: i32) {
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

    fn build_grid(section: &HashMap<String, String>) -> Grid {
        let grid = Grid::builder().column_spacing(25).row_spacing(5).build();

            let mut index_offset = 0;
            if let Some(login) = section.get("login") {
                AppUI::build_row(&grid, "login", login, 0);
                index_offset = index_offset + 1;
            }

            if let Some(password) = section.get("password") {
                AppUI::build_row(&grid, "password", password, 1);
                index_offset = index_offset + 1;
            }

            for (index, (key, value)) in section.iter().enumerate() {
                if key == "login" || key == "password" {
                    continue;
                }
                AppUI::build_row(&grid, key, value, (index as i32) + index_offset);
            }
        grid
    }
}

