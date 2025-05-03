use gtk::{traits::{BoxExt, ContainerExt, GridExt, ButtonExt}, Box, Button, Clipboard, Expander, Grid, Label};
use toml::Value;

pub struct AppUI {
    main_content: Box,
    credentials: Value,
}

impl AppUI {
    pub fn new(main_content: Box, credentials: Value) -> Self {
        Self { main_content, credentials }
    }

    pub fn populate_content(self: &Self) {
        if let Some(table) = self.credentials.as_table() {
            for (section_name, section) in table {

                let section_single = Expander::builder()
                    .use_markup(true)
                    .label(&format!("<b>{}</b>", section_name))
                    .expanded(false)
                    .build();

                let grid: Grid = self.build_grid(section);

                section_single.add(&grid);

                self.main_content.pack_start(&section_single, false, false, 5);
            }
        }
    }

    fn build_labels(self: &Self, grid: &Grid, name: &str, value: &str, index: i32) {
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
                addon = addon + 1;
            }

            if let Some(password) = section_table["password"].as_str() {
                self.build_labels(&grid, "password", password, 1);
                addon = addon + 1;
            }

            for (index, (key, value)) in section_table.iter().enumerate() {
                if key == "login" || key == "password" {
                    continue;
                }
                self.build_labels(&grid, key, value.as_str().unwrap(), (index as i32) + addon);
            }
        };
        grid
    }
}
