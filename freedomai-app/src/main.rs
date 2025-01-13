fn main() {
    // Initialize GTK
    gtk::init();

    // Create a window
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("FreedomAI");
    window.set_default_size(350, 70);

    // Create a text entry
    let entry = gtk::Entry::new();
    entry.set_placeholder_text("Enter input");

    // Create a button
    let button = gtk::Button::with_label("Run LLaMA");
    button.connect_clicked(move |_| {
        // Get the input from the text entry
        let input = entry.get_text().unwrap();
        let c_input = CString::new(input).unwrap();

        // Call the C++ function
        let output = unsafe { llama_run(c_input.as_ptr()) };

        // Print the output
        println!("Output: {}", unsafe { CString::from_raw(output).to_str().unwrap() });
    });

    // Add the entry and button to the window
    window.add(&gtk::Box::new(gtk::Orientation::Vertical, 0));
    window.add(&entry);
    window.add(&button);

    // Show the window and its contents
    window.show_all();

    // Run the GTK main loop
    gtk::main();
}




