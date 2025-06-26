use eframe::egui::{self, Style, Visuals, Sense, vec2, pos2};
use crate::gif_handler::GifHandler;

/// Dark or Light theme.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Theme {
    Dark,
    Light,
}
impl Theme {
    pub fn default_visuals(self) -> Visuals {
        match self {
            Self::Dark => Visuals::dark(),
            Self::Light => Visuals::light(),
        }
    }
    pub fn default_style(self) -> Style {
        Style { visuals: self.default_visuals(), ..Default::default() }
    }
}
// Automatically default to Dark theme
impl Default for Theme {
    fn default() -> Self { Theme::Dark }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SortingAlgorithm {
    QuickSort,
    Introsort,
    QuadSort,
    BurstSort,
    SpaghettiSort,
    Duck,
}
impl Default for SortingAlgorithm {
    fn default() -> Self { SortingAlgorithm::QuickSort }
}

#[derive(Default)]
pub struct SorthosApp {
    selected_algorithm: SortingAlgorithm,
    theme: Theme,
    duck_gif: GifHandler,
}

impl SorthosApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        
        // Try to load the duck gif from assets
        // Use path relative to cargo project root, or try multiple possible paths
        let possible_paths = [
            "src/assets/spinning-duck.gif",
            "assets/spinning-duck.gif", 
            "./src/assets/spinning-duck.gif",
        ];
        
        let mut loaded = false;
        for path_str in &possible_paths {
            let duck_path = std::path::Path::new(path_str);
            if duck_path.exists() {
                if let Err(e) = app.duck_gif.load_gif_from_file(&cc.egui_ctx, duck_path) {
                    eprintln!("Warning: Could not load duck gif from {}: {}", path_str, e);
                } else {
                    loaded = true;
                    break;
                }
            }
        }
        
        if !loaded {
            eprintln!("Warning: Could not find duck gif in any of the expected locations");
        }
        
        app
    }
    fn show_quicksort_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Quick Sort"); ui.separator();
        // add Quick Sort visualization here
    }
    fn show_introsort_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Introsort"); ui.separator();
        // add Introsort visualization here
    }
    fn show_quadsort_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Quad Sort"); ui.separator();
        // add Quad Sort visualization here
    }
    fn show_burstsort_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Burst Sort"); ui.separator();
        // add Burst Sort visualization here
    }
    fn show_spaghettisort_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Spaghetti Sort"); ui.separator();
        // add Spaghetti Sort visualization here
    }
    fn show_duck_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Duck");
        ui.separator();
        
        // Center the duck gif
        ui.vertical_centered(|ui| {
            self.duck_gif.render(ui, [128.0, 128.0]);
        });
    }
}

pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
    });
    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter().rect(rect, radius, visuals.bg_fill, visuals.bg_stroke, egui::StrokeKind::Inside);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = pos2(circle_x, rect.center().y);
        ui.painter().circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }
    response
}
#[expect(dead_code)]
fn toggle_ui_compact(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
    });
    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter().rect(rect, radius, visuals.bg_fill, visuals.bg_stroke, egui::StrokeKind::Inside);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = pos2(circle_x, rect.center().y);
        ui.painter().circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }
    response
}
pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}

impl eframe::App for SorthosApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_style(self.theme.default_style());
        // Top panel only shows the main title now
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("Sorthos");
        });
        egui::SidePanel::left("algorithm_selector").show(ctx, |ui| {
            // Side panel title with inline theme toggle
            ui.horizontal(|ui| {
                ui.heading("Light/Dark");
                let mut is_light = self.theme == Theme::Light;
                if ui.add(toggle(&mut is_light)).changed() {
                    self.theme = if is_light { Theme::Light } else { Theme::Dark };
                }
            });
            ui.separator();
            ui.selectable_label(matches!(self.selected_algorithm, SortingAlgorithm::QuickSort), "Quick Sort").clicked().then(|| self.selected_algorithm = SortingAlgorithm::QuickSort);
            ui.selectable_label(matches!(self.selected_algorithm, SortingAlgorithm::Introsort), "Introsort").clicked().then(|| self.selected_algorithm = SortingAlgorithm::Introsort);
            ui.selectable_label(matches!(self.selected_algorithm, SortingAlgorithm::QuadSort), "Quad Sort").clicked().then(|| self.selected_algorithm = SortingAlgorithm::QuadSort);
            ui.selectable_label(matches!(self.selected_algorithm, SortingAlgorithm::BurstSort), "Burst Sort").clicked().then(|| self.selected_algorithm = SortingAlgorithm::BurstSort);
            ui.selectable_label(matches!(self.selected_algorithm, SortingAlgorithm::SpaghettiSort), "Spaghetti Sort").clicked().then(|| self.selected_algorithm = SortingAlgorithm::SpaghettiSort);
            ui.selectable_label(matches!(self.selected_algorithm, SortingAlgorithm::Duck), "Duck").clicked().then(|| self.selected_algorithm = SortingAlgorithm::Duck);
        });
        egui::CentralPanel::default().show(ctx, |ui| match self.selected_algorithm {
            SortingAlgorithm::QuickSort => self.show_quicksort_page(ui),
            SortingAlgorithm::Introsort => self.show_introsort_page(ui),
            SortingAlgorithm::QuadSort => self.show_quadsort_page(ui),
            SortingAlgorithm::BurstSort => self.show_burstsort_page(ui),
            SortingAlgorithm::SpaghettiSort => self.show_spaghettisort_page(ui),
            SortingAlgorithm::Duck => self.show_duck_page(ui),
        });
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sorthos - Sorting Algorithms Visualizer",
        options,
        Box::new(|cc| Ok(Box::new(SorthosApp::new(cc)) as Box<dyn eframe::App>)),
    )
}
