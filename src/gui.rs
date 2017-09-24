use std::cell::RefCell;
use std::rc::Rc;

use gdk;
use gtk::prelude::*;
use gtk::{self, Button, DrawingArea, Inhibit, Orientation, Window, WindowType};

use grid::Grid;

struct Model {
    grid: Grid,
    scale: f64,
    run_simulation: bool,
}

pub fn create() {
    let model = Rc::new(RefCell::new(Model {
        grid: Grid::new(30, 30),
        scale: 50.0,
        run_simulation: false,
    }));

    let window = Window::new(WindowType::Toplevel);
    window.set_size_request(800, 800);
    window.set_title("Conway's Game of Life");
    window.set_resizable(false);

    let container = gtk::Box::new(Orientation::Vertical, 1);
    window.add(&container);

    let menu_container = gtk::Box::new(Orientation::Horizontal, 1);
    container.add(&menu_container);

    let drawing_area = DrawingArea::new();
    container.add(&drawing_area);
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);

    drawing_area.connect_draw(clone!(model, drawing_area => move |_, cr| {
        let mut model = model.borrow_mut();
        let (width, height) = (model.grid.width, model.grid.height);
        let widget_height = drawing_area.get_allocated_height();
        model.scale = (widget_height as usize / height) as f64;

        cr.set_source_rgb(0., 0., 0.);

        for y in 1..height {
            for x in 1..width {
                cr.rectangle(x as f64 * model.scale, y as f64 * model.scale, model.scale, model.scale);
                cr.set_line_width(0.8);

                let cell = model.grid.get(x, y);
                if cell.alive {
                    cr.fill_preserve();
                }

                cr.stroke();
            }
        }

        Inhibit(false)
    }));

    drawing_area.add_events(gdk::BUTTON_PRESS_MASK.bits() as i32);
    drawing_area.connect_button_press_event(clone!(drawing_area, model => move |_, event| {
        let mut model = model.borrow_mut();
        let (clicked_x, clicked_y) = event.get_position();

        let cell_x = (clicked_x / model.scale) as usize;
        let cell_y = (clicked_y / model.scale) as usize;
        model.grid.toggle_cell_alive(cell_x, cell_y);
        
        drawing_area.queue_draw();
        Inhibit(false)
    }));

    let button_container = gtk::Box::new(Orientation::Horizontal, 1);
    menu_container.pack_start(&button_container, true, false, 0);

    let next_gen_button = Button::new_with_label("Next");
    button_container.pack_start(&next_gen_button, false, false, 1);
    next_gen_button.connect_clicked(clone!(drawing_area, model => move |_| {
        model.borrow_mut().grid.next_generation();
        drawing_area.queue_draw();
    }));

    let start_simulation_button = Button::new_with_label("Start");
    button_container.pack_start(&start_simulation_button, false, false, 0);

    start_simulation_button.connect_clicked(clone!(drawing_area, model => move |_| {
        model.borrow_mut().run_simulation = true;

        gtk::timeout_add(500, clone!(drawing_area, model => move || {
            let mut model = model.borrow_mut();
            if model.run_simulation {
                model.grid.next_generation();
                drawing_area.queue_draw();
                gtk::Continue(true)
            } else {
                gtk::Continue(false)
            }
        }));
    }));

    let stop_simulation_button = Button::new_with_label("Stop");
    button_container.pack_start(&stop_simulation_button, false, false, 0);
    stop_simulation_button.connect_clicked(clone!(model => move |_| {
        model.borrow_mut().run_simulation = false;
    }));

    let clear_button = Button::new_with_label("Clear");
    button_container.pack_start(&clear_button, false, false, 0);
    clear_button.connect_clicked(clone!(drawing_area, model => move |_| {
        let mut model = model.borrow_mut();
        for cell in &mut model.grid.cells {
            cell.alive = false;
        }

        drawing_area.queue_draw();
    }));

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
}
