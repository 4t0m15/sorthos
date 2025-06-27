use eframe::egui::{self, Style, Visuals, Sense, vec2, pos2};
use crate::gif_handler::GifHandler;
use crate::gui::bars_render::SortVisualizerApp;

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
    About,
    Duck,
}
impl Default for SortingAlgorithm {
    fn default() -> Self { SortingAlgorithm::About }
}

pub struct Sorthos {
    selected_algorithm: SortingAlgorithm,
    theme: Theme,
    duck_gif: GifHandler,
    sort_app: SortVisualizerApp,
}

impl Default for Sorthos {
    fn default() -> Self {
        Self {
            selected_algorithm: SortingAlgorithm::default(),
            theme: Theme::default(),
            duck_gif: GifHandler::default(),
            sort_app: SortVisualizerApp::new(100, crate::sorting::SortingAlgorithm::Quick),
        }
    }
}
impl Sorthos {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        
        // Embed the duck gif at compile time
        const DUCK_GIF: &[u8] = include_bytes!("../assets/spinning-duck.gif");
        if let Err(_e) = app.duck_gif.load_gif_from_bytes(&cc.egui_ctx, DUCK_GIF, "duck") {
            print!("Duck gif failed to load.");
        }
        // Ensure bars use theme colors (white for Dark) on first launch
        app.sort_app.apply_theme(app.theme);
        app
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

impl eframe::App for Sorthos {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_style(self.theme.default_style());
        // Top panel only shows the main title now
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("Sorthos");
        });
        egui::SidePanel::left("algorithm_selector").show(ctx, |ui| {
            // Side panel title with inline theme toggle
            ui.horizontal(|ui| {
                ui.heading("Dark/Light");
                let mut is_light = self.theme == Theme::Light;
                if ui.add(toggle(&mut is_light)).changed() {
                    self.theme = if is_light { Theme::Light } else { Theme::Dark };
                    // Reset and reapply theme to sort_app for consistent bar colors
                    self.sort_app.reset_bars();
                    self.sort_app.apply_theme(self.theme);
                }
            });
            ui.separator();
            ui.selectable_label(self.selected_algorithm == SortingAlgorithm::About, "Sort").clicked().then(|| self.selected_algorithm = SortingAlgorithm::About);
            ui.selectable_label(self.selected_algorithm == SortingAlgorithm::Duck, "Duck").clicked().then(|| self.selected_algorithm = SortingAlgorithm::Duck);
        });
        if self.selected_algorithm == SortingAlgorithm::About {
            // Delegate full UI drawing to sort_app
            self.sort_app.update(ctx, frame);
        } else {
            egui::CentralPanel::default().show(ctx, |ui| match self.selected_algorithm {
                SortingAlgorithm::Duck => self.show_duck_page(ui),
                _ => {},
            });
        }
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
        Box::new(|cc| Ok(Box::new(Sorthos::new(cc)) as Box<dyn eframe::App>)),
    )
}
