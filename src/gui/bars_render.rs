use crate::models::SortBar;
use crate::sorting::{start_sort, Operation, SortingAlgorithm};
use eframe::egui::{self, Color32, Slider};
use rand::seq::SliceRandom;
use std::sync::mpsc;
use std::thread;

pub struct SortVisualizerApp {
    bars: Vec<SortBar>,
    algorithm: SortingAlgorithm,
    num_bars: usize,
    sorting: bool,
    rx: mpsc::Receiver<Operation>,
    tx: mpsc::Sender<Operation>,
    draw_circle: bool,
}

impl SortVisualizerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut app = Self {
            bars: Vec::new(),
            algorithm: SortingAlgorithm::Bubble,
            num_bars: 128,
            sorting: false,
            tx,
            rx,
            draw_circle: false,
        };
        app.reset_bars();
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.request_repaint(); // continuous repaint
        app
    }

    fn reset_bars(&mut self) {
        self.bars = (0..self.num_bars).map(SortBar::new).collect();
    }

    fn shuffle_bars(&mut self) {
        self.bars.shuffle(&mut rand::thread_rng());
    }

    fn start_sorting(&mut self) {
        if self.sorting {
            return;
        }
        self.sorting = true;
        let algo = self.algorithm;
        let bars_clone = self.bars.clone();
        let tx = self.tx.clone();
        start_sort(algo, bars_clone, tx);
    }

    fn handle_ops(&mut self) {
        while let Ok(op) = self.rx.try_recv() {
            match op {
                Operation::Compare(i, j) => {
                    self.bars[i].color = Color32::YELLOW;
                    self.bars[j].color = Color32::YELLOW;
                }
                Operation::Swap(i, j) => {
                    self.bars.swap(i, j);
                    self.bars[i].color = Color32::GREEN;
                    self.bars[j].color = Color32::GREEN;
                }
                Operation::SetColor(i, col) => {
                    self.bars[i].color = col;
                }
                Operation::Done => {
                    self.sorting = false;
                }
            }
        }
    }
}

impl eframe::App for SortVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // First, handle incoming sort operations:
        self.handle_ops();

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.heading("Sorthos - Bar Visualization");
        });

        egui::SidePanel::left("side").show(ctx, |ui| {
            ui.label("Algorithm:");
            for &alg in SortingAlgorithm::all() {
                if ui.selectable_label(self.algorithm == alg, format!("{alg:?}")).clicked() {
                    self.algorithm = alg;
                }
            }
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Shuffle").clicked() && !self.sorting {
                    self.shuffle_bars();
                }
                if ui.button("Sort").clicked() && !self.sorting {
                    self.start_sorting();
                }
            });

            ui.add(
                Slider::new(&mut self.num_bars, 16..=512)
                    .text("bars")
                    .clamp_to_range(true),
            )
            .on_hover_text("Change number of bars");
            if ui.button("Reset").clicked() && !self.sorting {
                self.reset_bars();
            }

            ui.checkbox(&mut self.draw_circle, "Circle style");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let rect = ui.available_rect_before_wrap();
            let n = self.bars.len() as f32;
            let bar_w = rect.width() / n;

            for (i, bar) in self.bars.iter().enumerate() {
                let x = rect.left() + i as f32 * bar_w;
                let h = rect.height() * (bar.value as f32 + 1.0) / n;
                let y = rect.bottom() - h;

                if self.draw_circle {
                    // For demonstration, still using filled rectangles
                    painter.rect_filled(
                        egui::Rect::from_min_size(
                            egui::pos2(x, y),
                            egui::vec2(bar_w - 1.0, h),
                        ),
                        0.0,
                        bar.color,
                    );
                } else {
                    painter.rect_filled(
                        egui::Rect::from_min_size(
                            egui::pos2(x, y),
                            egui::vec2(bar_w - 1.0, h),
                        ),
                        0.0,
                        bar.color,
                    );
                }
            }
        });

        // keep repainting at ~60 fps during sort
        if self.sorting {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
    }
}

pub fn run_sort_visualizer() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sorthos - Sorting Algorithm Visualizer",
        options,
        Box::new(|cc| Ok(Box::new(SortVisualizerApp::new(cc)) as Box<dyn eframe::App>)),
    )
}