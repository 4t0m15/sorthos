use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::time::Duration;

pub fn tim_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 {
            // Highlight comparison
            let _ = tx.send(Operation::Compare(j - 1, j));
            std::thread::sleep(Duration::from_millis(6));

            if bars[j - 1].value > bars[j].value {
                // Swap and animate
                let _ = tx.send(Operation::Swap(j - 1, j));
                bars.swap(j - 1, j);
                std::thread::sleep(Duration::from_millis(6));
            } else {
                // Reset colours and break early
                let _ = tx.send(Operation::SetColor(j - 1, Color32::WHITE));
                let _ = tx.send(Operation::SetColor(j, Color32::WHITE));
                break;
            }
            j -= 1;
        }
    }
    // Paint all bars white when done
    for idx in 0..n {
        let _ = tx.send(Operation::SetColor(idx, Color32::WHITE));
    }
}
