use gtk::{self, BoxExt, Button, Orientation};

pub struct Menu {
    pub container: gtk::Box,
    pub start_button: Button,
    pub stop_button: Button,
    pub next_button: Button,
    pub clear_button: Button,
}

impl Menu {
    pub fn new() -> Menu {
        let container = gtk::Box::new(Orientation::Horizontal, 1);

        let next_button = Button::new_with_label("Next");
        container.pack_start(&next_button, false, false, 1);

        let start_button = Button::new_with_label("Start");
        container.pack_start(&start_button, false, false, 0);

        let stop_button = Button::new_with_label("Stop");
        container.pack_start(&stop_button, false, false, 0);

        let clear_button = Button::new_with_label("Clear");
        container.pack_start(&clear_button, false, false, 0);

        Menu {
            container,
            next_button,
            start_button,
            stop_button,
            clear_button,
        }
    }
}
