mod cli_parser;

use gtk4 as gtk;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use cli_parser::Args;
use clap::Parser;

fn main() -> glib::ExitCode {

    // Parse CLI Arguments
    let args = Args::parse();

    // Print Hello World
    println!("Hello, world!");

    // Build GTK Application
    let app = Application::builder()
        .application_id("org.mwglen.mvt")
        .build();

    // Create a Window
    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Show the window.
        window.show();
    });

    // Run the App
    app.run()

}
