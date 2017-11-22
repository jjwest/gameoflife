#![feature(proc_macro)]

extern crate gdk;
extern crate gtk;

mod grid;
mod app;
mod menu;
mod grid_widget;

use std::cell::RefCell;
use std::rc::Rc;

use gtk::{ButtonExt, WidgetExt};

use app::App;
use grid::Grid;

pub struct Model {
    pub grid: Grid,
    pub scale: f64,
    pub run_simulation: bool,
}


fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK");
        std::process::exit(1);
    }

    let model = Rc::new(RefCell::new(Model {
        grid: Grid::new(30, 30),
        scale: 50.0,
        run_simulation: false,
    }));

    let app = App::new(model.clone());

    {
        // Setup next generation button
        let model = model.clone();
        let drawing_area = app.grid.drawing_area.clone();

        app.menu.next_button.connect_clicked(move |_| {
            model.borrow_mut().grid.next_generation();
            drawing_area.queue_draw();
        });
    }

    {
        // Setup start button
        let model = model.clone();
        let drawing_area = app.grid.drawing_area.clone();

        app.menu.start_button.connect_clicked(move |_| {
            model.borrow_mut().run_simulation = true;
            let model = model.clone();
            let drawing_area = drawing_area.clone();

            gtk::timeout_add(500, move || {
                let mut model = model.borrow_mut();
                if model.run_simulation {
                    model.grid.next_generation();
                    drawing_area.queue_draw();
                    gtk::Continue(true)
                } else {
                    gtk::Continue(false)
                }
            });
        });
    }

    {
        // Setup stop button
        let model = model.clone();
        app.menu.stop_button.connect_clicked(move |_| {
            model.borrow_mut().run_simulation = false;
        });
    }

    {
        // Setup clear button
        let model = model.clone();
        let drawing_area = app.grid.drawing_area.clone();

        app.menu.clear_button.connect_clicked(move |_| {
            let mut model = model.borrow_mut();
            for cell in &mut model.grid {
                cell.alive = false;
            }

            drawing_area.queue_draw();
        });
    }



    app.window.show_all();

    gtk::main();
}
