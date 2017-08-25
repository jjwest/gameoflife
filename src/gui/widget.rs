use cairo::Context;
use gtk::prelude::*;
use gtk;
use relm;
use relm_attributes::widget;

use grid::Grid;

#[derive(Clone, Debug)]
pub struct Model {
    grid: Grid,
    scale: i32,
}

#[derive(Msg)]
pub enum Event {
    Draw(Context),
    Tick,
    Quit,
}

#[widget]
impl relm::Widget for Widget {
    fn model() -> Model {
        Model {
            grid: Grid::new(30, 30),
            scale: 1,
        }
    }

    fn update(&mut self, event: Event) {
        match event {
            Event::Draw(context) => self.draw_grid(context),
            Event::Tick => {
                self.model.grid.next_generation();
                self.drawing_area.queue_draw();
            }
            Event::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            title: "Conway's Game of Life",
            property_default_height: 800,
            property_default_width: 800,

            gtk::Box {
                orientation: gtk::Orientation::Horizontal,

                gtk::Box {
                    orientation: gtk::Orientation::Vertical,

                    gtk::Button {
                        label: "Next",
                        clicked => Event::Tick,
                    }
                },

                #[name="drawing_area"]
                gtk::DrawingArea {
                    hexpand: true,
                    vexpand: true,
                    draw(_, cr) => (Event::Draw(cr.clone()), gtk::Inhibit(false)),
                }
            },
            
            delete_event(_, _) => (Event::Quit, gtk::Inhibit(false)),
        }
    }
}

impl Widget {
    fn draw_grid(&self, cr: Context) {
        let cell_size = 10.;
        let grid_width = self.model.grid.width;

        cr.set_source_rgb(0., 0.0, 0.);
        cr.rectangle(200., 200., cell_size, cell_size);
        cr.fill();

        for y in 0..self.model.grid.height {
            for x in 0..self.model.grid.width {
                if self.model.grid.cells[y * grid_width + x].alive {
                    println!("INSIDE");
                    cr.rectangle((x + 5) as f64, (y + 5) as f64, cell_size, cell_size);
                    cr.fill();
                }
            }
        }
    }
}
