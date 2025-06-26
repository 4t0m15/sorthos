use eframe::egui;

#[derive(Default)]
pub struct SorthosApp {
    // Add your application state here
}

impl SorthosApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
    }
}

impl eframe::App for SorthosApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sorthos");
            ui.separator();
        });
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Sorthos"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Sorthos",
        options,
        Box::new(|cc| Ok(Box::new(SorthosApp::new(cc)))),
    )
}