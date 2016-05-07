/// Diese Includes sind alle nötig
extern crate gdk;
extern crate gtk;
use gdk::enums::*;
use gtk::prelude::*;

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
    let stack_switcher = gtk::StackSwitcher::new();
    box_main.pack_start(&stack_switcher, false, false, 0);

    let stack = gtk::Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
    stack_switcher.set_stack(Some(&stack));

    let separator = gtk::Separator::new(gtk::Orientation::Vertical);
    box_main.pack_start(&separator, false, false, 0);

    box_main.pack_start(&stack, true, true, 0);
    // Construct the StackSwitcher
    for i in 1..10 {
        let label = gtk::Label::new(Some(&i.to_string()));
        stack.add_named(&label, &i.to_string());
    }
    window.add(&box_main);

    // Swipe
    let swipe = gtk::GestureSwipe::new(&stack);
    swipe.connect_swipe(move |_swipe, swipe_x, _swipe_y| {
        match swipe_x < 0f64 {
            true => {stack.set_visible_child_full("2", gtk::StackTransitionType::SlideLeftRight)},
            false => {stack.set_visible_child_full("1", gtk::StackTransitionType::SlideLeftRight)},
        };
    });

    window.show_all();

    gtk::main();
}
