use crate::models::SortBar;
use eframe::egui::Color32;
use std::sync::mpsc;

/// Visual Selection Sort implementation
pub fn selection_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<crate::sorting::Operation>) {
    let n = bars.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in i + 1..n {
            let _ = tx.send(crate::sorting::Operation::Compare(min_idx, j));
            std::thread::sleep(std::time::Duration::from_millis(10));

            if bars[j].value < bars[min_idx].value {
                min_idx = j;
            }

            let _ = tx.send(crate::sorting::Operation::SetColor(min_idx, Color32::WHITE));
            let _ = tx.send(crate::sorting::Operation::SetColor(j, Color32::WHITE));
        }

        if min_idx != i {
            let _ = tx.send(crate::sorting::Operation::Swap(i, min_idx));
            bars.swap(i, min_idx);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
