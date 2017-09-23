#![feature(proc_macro)]

extern crate gdk;
extern crate gtk;

#[macro_use]
mod macros;
mod gui;
mod grid;

fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK");
        std::process::exit(1);
    }

    gui::create();
    gtk::main();
}
