mod gui;

fn main() -> Result<(), eframe::Error> {
    gui::gui_main::run_gui()
}