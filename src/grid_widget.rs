use std::cell::RefCell;
use std::rc::Rc;

use gtk::{DrawingArea, WidgetExt, Inhibit};
use gdk;

use Model;

pub struct GridWidget {
    pub drawing_area: DrawingArea,
}

impl GridWidget {
    pub fn new(model: Rc<RefCell<Model>>) -> GridWidget {
        let drawing_area = DrawingArea::new();
        drawing_area.set_hexpand(true);
        drawing_area.set_vexpand(true);
        drawing_area.add_events(gdk::BUTTON_PRESS_MASK.bits() as i32);

        {
            // Setup the drawing
            let model = model.clone();
            let drawing_area_clone = drawing_area.clone();

            drawing_area.connect_draw(move |_, cr| {
                let mut model = model.borrow_mut();
                let (grid_width, grid_height) = (model.grid.get_width(), model.grid.get_height());
                let widget_height = drawing_area_clone.get_allocated_height();
                model.scale = (widget_height as usize / grid_height) as f64;

                cr.set_source_rgb(0., 0., 0.);

                for y in 1..grid_height {
                    for x in 1..grid_width {
                        cr.rectangle(
                            x as f64 * model.scale,
                            y as f64 * model.scale,
                            model.scale,
                            model.scale,
                        );
                        cr.set_line_width(0.8);

                        let cell = model.grid[(x, y)];
                        if cell.alive {
                            cr.fill_preserve();
                        }

                        cr.stroke();
                    }
                }

                Inhibit(false)
            });
        }
        {
            // Setup toggling cells by clicking
            let drawing_area_clone = drawing_area.clone();
            let model = model.clone();

            drawing_area.connect_button_press_event(move |_, event| {
                let mut model = model.borrow_mut();
                let (clicked_x, clicked_y) = event.get_position();

                let cell_x = (clicked_x / model.scale) as usize;
                let cell_y = (clicked_y / model.scale) as usize;
                let cell = &mut model.grid[(cell_x, cell_y)];
                cell.alive = !cell.alive;

                drawing_area_clone.queue_draw();
                Inhibit(false)
            });
        }

        GridWidget { drawing_area }
    }
}
