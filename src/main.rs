use gtk::prelude::*;
use gtk::Application;
use crate::ui::build_credential_list_ui; // Import the main UI building function

mod ui; // Declare the ui module
mod data; // Declare the data module

const APP_ID: &str = "com.example.Rustillum";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_credential_list_ui);

    // Run the application
    app.run();
}
