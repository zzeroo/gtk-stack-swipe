// Diese Includes sind alle nötig
extern crate gdk;
extern crate gtk;
use gdk::enums::*;
use gtk::prelude::*;
mod stack;
use stack::Stack;
fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initalize GTK."));
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    // Window properties
    window.set_title("Stack switcher test");
    window.set_default_size(800, 600);

    // Connect delete event to quit the gtk::main thread
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });
    // Connect ESC key press event, and quit the gui if ESC was pressed
    window.connect_key_press_event(move |_, key| {
        match key.get_keyval() as u32 {
            key::Escape => gtk::main_quit(),
            _ => (),
        }
        Inhibit(false)
    });

    let box_main = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let mut stack = Stack::new();
    box_main.pack_start(&stack.stack, true, true, 0);

    // Construct the StackSwitcher
    for i in &["Sensor 1", "Sensor 2", "Sensor 3", "Einstellungen"] {
        stack.create_windows(&i.to_string());
    }
    window.add(&box_main);

    // Swipe
    let swipe = gtk::GestureSwipe::new(&stack.stack);
    swipe.connect_swipe(move |_swipe, swipe_x, _swipe_y| {
        match swipe_x < 0f64 {
            true => {stack.next_window()},
            false => {stack.prev_window()},
        };
    });

    window.show_all();

    gtk::main();
}
