use std::cell::RefCell;
use std::rc::Rc;

use gtk::{self, ContainerExt, Window, WindowExt, WindowType, WidgetExt, Orientation};

use grid_widget::GridWidget;
use menu::Menu;
use Model;

pub struct App {
    pub window: Window,
    pub container: gtk::Box,
    pub menu: Menu,
    pub grid: GridWidget,
}

impl App {
    pub fn new(model: Rc<RefCell<Model>>) -> App {
        let window = Window::new(WindowType::Toplevel);
        window.set_size_request(800, 800);
        window.set_title("Conway's Game of Life");
        window.set_resizable(false);

        let container = gtk::Box::new(Orientation::Vertical, 1);
        window.add(&container);

        let menu = Menu::new();
        container.add(&menu.container);

        let grid = GridWidget::new(model);
        container.add(&grid.drawing_area);

        window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });


        App {
            window,
            container,
            menu,
            grid,
        }
    }
}
