use crate::models::SortBar;
use crate::sorting::{start_sort, Operation, SortingAlgorithm};
use eframe::egui::{self, Color32, Slider};
use rand::seq::SliceRandom;
use std::sync::mpsc;
use crate::gui_backend::gui::Theme;
use crate::gui::check_theme_consistancy::apply_theme_consistency;

pub struct SortVisualizerApp {
    bars: Vec<SortBar>,
    algorithm: SortingAlgorithm,
    num_bars: usize,
    sorting: bool,
    rx: mpsc::Receiver<Operation>,
    tx: mpsc::Sender<Operation>,
    current_theme: Theme,
}

impl SortVisualizerApp {
    pub fn reset_bars(&mut self) {
        // Abandon any in-progress sort by replacing the channel and clearing sorting flag
        let (new_tx, new_rx) = mpsc::channel();
        self.tx = new_tx;
        self.rx = new_rx;
        self.sorting = false;

        self.bars = (0..self.num_bars).map(SortBar::new).collect();
        // Apply current theme to newly reset bars
        self.apply_theme(self.current_theme);
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
        // Clone and enforce correct bar colors before starting
        let mut bars_clone = self.bars.clone();
        apply_theme_consistency(&mut bars_clone, self.current_theme);
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
                    // remap "WHITE reset" to your theme’s default background color
                    let default = match self.current_theme {
                        Theme::Light => Color32::BLACK,
                        Theme::Dark  => Color32::WHITE,
                    };
                    self.bars[i].color = if col == Color32::WHITE { default } else { col };
                }
                Operation::Done => {
                    self.sorting = false;
                }
            }
        }
    }

    /// Apply the current theme to all bar colors.
    pub fn apply_theme(&mut self, theme: Theme) {
        // Store and apply theme
        self.current_theme = theme;
        apply_theme_consistency(&mut self.bars, theme);
    }

    /// Create a new SortVisualizerApp with given number of bars and initial algorithm.
    pub fn new(num_bars: usize, algorithm: SortingAlgorithm) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut app = Self {
            bars: Vec::new(),
            algorithm,
            num_bars,
            sorting: false,
            tx,
            rx,
            current_theme: Theme::Light, // default, will be applied below
        };
        // Initialize bars with default values and apply theme
        app.reset_bars();
        app
    }
}

impl eframe::App for SortVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // First, handle incoming sort operations:
        self.handle_ops();

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
                Slider::new(&mut self.num_bars, 16..=315)
                    .text("bars"),
            )
            .on_hover_text("Change number of bars");
            if ui.button("Reset").clicked() && !self.sorting {
                self.reset_bars();
            }
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

                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(x, y),
                        egui::vec2(bar_w - 1.0, h),
                    ),
                    0.0,
                    bar.color,
                );
            }
        });

        // keep repainting at ~60 fps during sort
        if self.sorting {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
    }
}