use gtk::prelude::{ApplicationExt, ApplicationExtManual, BuilderExtManual, GtkWindowExt, WidgetExt};
use gtk::{Application, ApplicationWindow, Builder, TreeView};

const APP_ID: &str = "org.rustillum.ui";

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to "activate" signal of application
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Load the UI from the glade file
    let builder = Builder::from_file("ui/rustillum.glade");

    // Get the window from the builder
    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(app));

    // Get the TreeView from the builder
    let tree_view: TreeView = builder.object("tree_view").expect("Couldn't get tree_view");

    // Present window
    window.show_all();
}
