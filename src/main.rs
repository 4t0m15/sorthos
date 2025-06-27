mod gif_handler;
mod gui;
mod gui_backend;
mod models;
mod sorting;

fn main() -> Result<(), eframe::Error> {
    gui_backend::gui::run_gui()
}