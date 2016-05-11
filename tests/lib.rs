extern crate gtk_stack_swipe;
extern crate gtk;

use gtk_stack_swipe::stack::Stack;

fn setup() {
    gtk::init();
}



#[test]
fn test_get_window_index() {
    gtk::init();
    let mut stack = Stack::new();
    for w in &["Fenster 1", "Fenster 2", "Einstellungen"] {
        stack.create_windows(&w.to_string());
    }
    assert_eq!(&stack.get_window_index("Einstellungen"), &Some(2));
    gtk::main_quit();
}

#[test]
fn test_get_next_window() {
    gtk::init();
    let mut stack = Stack::new();
    for w in &["Fenster 1", "Fenster 2", "Einstellungen"] {
        stack.create_windows(&w.to_string());
    }
    assert_eq!(&stack.get_window_index("Einstellungen"), &Some(2));
    gtk::main_quit();
}

#[test]
#[ignore]
fn test_get_prev_window() {
    unimplemented!();
}
