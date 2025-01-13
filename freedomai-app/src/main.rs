use gtk::prelude::*;
use gtk::init;

fn main() {
    // Initialize GTK
    init();

    // Create a window
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("FreedomAI");
    window.set_default_size(350, 70);

    // Create a button
    let button = gtk::Button::with_label("Click me!");
    button.connect_clicked(move |_| {
        println!("Button clicked!");
    });

    // Add the button to the window
    window.add(&button);

    // Show the window and its contents
    window.show_all();

    // Run the GTK main loop
    gtk::main();
}

