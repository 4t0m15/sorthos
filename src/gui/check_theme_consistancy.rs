use crate::models::SortBar;
use crate::gui_backend::gui::Theme;
use eframe::egui::Color32;

/// Ensures all bars are colored according to the current theme (light => black, dark => white).
pub fn ensure_theme_consistency(bars: &mut [SortBar], theme: Theme) {
    let default_color = match theme {
        Theme::Light => Color32::BLACK,
        Theme::Dark => Color32::WHITE,
    };
    for bar in bars {
        bar.color = default_color;
    }
}
