#![feature(proc_macro)]

extern crate cairo;
extern crate gdk;
extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

use relm::Widget;

mod gui;
mod grid;

use gui::Window;

fn main() {
    Window::run(()).unwrap();
}
