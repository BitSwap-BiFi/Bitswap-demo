extern crate gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Orientation};

fn main() {
    let application = Application::new(
        Some("com.example.dexapp"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("DEX GUI");
        window.set_default_size(400, 200);

        let vbox = Box::new(Orientation::Vertical, 5);

        // Swap Button
        let swap_button = Button::with_label("Swap");
        swap_button.connect_clicked(|_| {
            println!("Swap function triggered!");
            // Implement the swap functionality here
        });
        vbox.add(&swap_button);

        // Liquidity Provision (LP) Button
        let lp_button = Button::with_label("Add Liquidity");
        lp_button.connect_clicked(|_| {
            println!("Liquidity Provision function triggered!");
            // Implement the liquidity provision functionality here
        });
        vbox.add(&lp_button);

        // Contract Interaction Button
        let contract_button = Button::with_label("Interact with Contract");
        contract_button.connect_clicked(|_| {
            println!("Contract interaction function triggered!");
            // Implement the contract interaction functionality here
        });
        vbox.add(&contract_button);

        window.add(&vbox);
        window.show_all();
    });

    application.run(&[]);
}
