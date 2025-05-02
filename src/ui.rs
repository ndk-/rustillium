use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Expander, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow};
use toml::Value;
use crate::data::load_credential_data; // Import from the data module

const CREDENTIAL_FILE_PATH: &str = "data/cre.toml"; // Define a constant for the file path

// Function to build the main application UI
pub fn build_credential_list_ui(app: &Application) {
    // Load the credential data
    let credential_sections = load_credential_data(CREDENTIAL_FILE_PATH).expect("Unable to load credential data");

    // Create a list box to hold the sections
    let section_list_box = ListBox::new();

    // Iterate through the sections in the TOML data
    for (section_name, section_data) in credential_sections.iter() {
        // Build the UI for the credential section
        let section_row = build_credential_section_ui(section_name, section_data);
        section_list_box.add(&section_row);
    }

    // Create a scrolled window to hold the list box
    let scrolled_window = ScrolledWindow::builder().build();
    scrolled_window.set_child(Some(&section_list_box));

    // Create a new window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rustillum")
        .default_width(400)
        .default_height(300)
        .child(&scrolled_window)
        .build();

    // Present the window and all its children
    window.show_all();
}

// Function to build a single credential section UI
fn build_credential_section_ui(section_name: &str, section_data: &Value) -> ListBoxRow {
        // Create a new list box row for the section
        let section_row = ListBoxRow::new();

        // Create an expander for the section
        let expander = Expander::new(Some(section_name));
        expander.set_margin_start(10); // Add some indentation
        expander.set_use_markup(true); // Enable markup for the label

        // Create a label for the section name
        let section_label = Label::new(Some(section_name));
        section_label.set_halign(gtk::Align::Start);

        // Set the section label as the expander's label widget
        expander.set_label_widget(Some(&section_label));

        // Create a vertical box to hold the details within the expander
        let credential_details_box = Box::new(Orientation::Vertical, 5); // 5px spacing
        credential_details_box.set_margin_start(10); // Indent details further

        // Iterate through the key-value pairs
        if let Value::Table(credential_entries) = section_data {
            for (credential_key, credential_value) in credential_entries.iter() {
                // Format the key-value pair with Pango markup for bold key
                let credential_detail_text = format!("<b>{}</b>: {}", credential_key, credential_value.to_string());
                let credential_detail_label = Label::new(None); // Create label without initial text
                credential_detail_label.set_markup(&credential_detail_text); // Set text with markup
                credential_detail_label.set_halign(gtk::Align::Start); // Align text to the left
                // Use pack_start for GTK3 Box
                credential_details_box.pack_start(&credential_detail_label, false, false, 0);
            }
        }

        // Add the details box to the expander
        expander.set_child(Some(&credential_details_box));

        // Add the expander to the row
        section_row.set_child(Some(&expander));

        section_row
}
