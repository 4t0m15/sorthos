use crate::models::SortBar;
use eframe::egui::Color32;
use std::sync::mpsc;

/// Visual Insertion Sort implementation
pub fn insertion_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<crate::sorting::Operation>) {
    let n = bars.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 {
            let _ = tx.send(crate::sorting::Operation::Compare(j - 1, j));
            std::thread::sleep(std::time::Duration::from_millis(10));

            if bars[j - 1].value > bars[j].value {
                let _ = tx.send(crate::sorting::Operation::Swap(j - 1, j));
                bars.swap(j - 1, j);
                std::thread::sleep(std::time::Duration::from_millis(10));
                j -= 1;
            } else {
                break;
            }

            let _ = tx.send(crate::sorting::Operation::SetColor(j, Color32::WHITE));
            let _ = tx.send(crate::sorting::Operation::SetColor(j + 1, Color32::WHITE));
        }
    }
}
