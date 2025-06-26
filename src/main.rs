mod gui_backend;
use gui_backend::gui;

fn main() -> Result<(), eframe::Error> {
    gui::run_gui()
}