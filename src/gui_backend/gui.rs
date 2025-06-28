use eframe::egui::{self, Style, Visuals, Sense, vec2, pos2};
use crate::gif_handler::GifHandler;
use crate::gui::bars_render::SortVisualizerApp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Theme {
    #[default]
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum SortingAlgorithm {
    #[default]
    About,
    Duck,
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
        const DUCK_GIF: &[u8] = include_bytes!("../assets/spinning-duck.gif");
        let _ = app.duck_gif.load_gif_from_bytes(&cc.egui_ctx, DUCK_GIF, "duck");
        app.sort_app.apply_theme(app.theme);
        app
    }
    fn show_duck_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Duck");
        ui.separator();
        ui.vertical_centered(|ui| {
            self.duck_gif.render(ui, [128.0, 128.0]);
        });
    }
}

fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(size, Sense::click());
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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| ui.heading("Sorthos"));
        egui::SidePanel::left("algorithm_selector").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Dark/Light");
                let mut is_light = self.theme == Theme::Light;
                if ui.add(toggle(&mut is_light)).changed() {
                    self.theme = if is_light { Theme::Light } else { Theme::Dark };
                    self.sort_app.reset_bars();
                    self.sort_app.apply_theme(self.theme);
                }
            });
            ui.separator();
            if ui.selectable_label(self.selected_algorithm == SortingAlgorithm::About, "sorting").clicked() {
                self.selected_algorithm = SortingAlgorithm::About;
            }
            if ui.selectable_label(self.selected_algorithm == SortingAlgorithm::Duck, "duck").clicked() {
                self.selected_algorithm = SortingAlgorithm::Duck;
            }
        });
        match self.selected_algorithm {
            SortingAlgorithm::About => self.sort_app.update(ctx, frame),
            SortingAlgorithm::Duck => { egui::CentralPanel::default().show(ctx, |ui| self.show_duck_page(ui)); },
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
