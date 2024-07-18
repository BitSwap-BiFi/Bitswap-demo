extern crate gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let application = Application::new(
        Some("com.example.mygtkapp"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("My GTK App");
        window.set_default_size(350, 70);

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Button clicked!");
        });

        window.add(&button);
        window.show_all();
    });

    application.run(&[]);
}
