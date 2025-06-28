use super::sort_utils::insertion_sort_range_visual;
use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::time::Duration;

pub fn tim_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    if n <= 1 {
        return;
    }

    const MIN_MERGE: usize = 32;

    // For small arrays, use insertion sort
    if n < MIN_MERGE {
        insertion_sort_range_visual(bars, 0, n, tx);
    } else {
        // For larger arrays, use a hybrid approach with insertion sort for small runs
        // and merge sort for combining them
        let mut run_size = MIN_MERGE;

        // Sort individual subarrays of size MIN_MERGE using insertion sort
        let mut left = 0;
        while left < n {
            let right = std::cmp::min(left + MIN_MERGE, n);
            insertion_sort_range_visual(bars, left, right, tx);
            left = right;
        }

        // Start merging from size MIN_MERGE
        while run_size < n {
            let mut left = 0;
            while left < n {
                let mid = left + run_size;
                let right = std::cmp::min(left + 2 * run_size, n);

                if mid < right {
                    merge_ranges(bars, left, mid, right, tx);
                }
                left = right;
            }
            run_size *= 2;
        }
    }

    // Paint all bars white when done
    for idx in 0..n {
        let _ = tx.send(Operation::SetColor(idx, Color32::WHITE));
    }
}

fn merge_ranges(
    bars: &mut Vec<SortBar>,
    left: usize,
    mid: usize,
    right: usize,
    tx: &mpsc::Sender<Operation>,
) {
    // Create temporary storage for the merge
    let left_part: Vec<_> = bars[left..mid].iter().cloned().collect();
    let right_part: Vec<_> = bars[mid..right].iter().cloned().collect();

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    // Highlight the ranges being merged
    for idx in left..right {
        let _ = tx.send(Operation::SetColor(idx, Color32::YELLOW));
    }
    std::thread::sleep(Duration::from_millis(50));

    // Merge the two sorted halves
    while i < left_part.len() && j < right_part.len() {
        // Visual comparison
        let _ = tx.send(Operation::Compare(k, k));
        std::thread::sleep(Duration::from_millis(15));

        if left_part[i].value <= right_part[j].value {
            bars[k] = left_part[i].clone();
            let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
            i += 1;
        } else {
            bars[k] = right_part[j].clone();
            let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
            j += 1;
        }
        std::thread::sleep(Duration::from_millis(15));
        k += 1;
    }

    // Copy remaining elements
    while i < left_part.len() {
        bars[k] = left_part[i].clone();
        let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
        i += 1;
        k += 1;
        std::thread::sleep(Duration::from_millis(10));
    }

    while j < right_part.len() {
        bars[k] = right_part[j].clone();
        let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
        j += 1;
        k += 1;
        std::thread::sleep(Duration::from_millis(10));
    }

    // Reset colors after merge
    for idx in left..right {
        let _ = tx.send(Operation::SetColor(idx, Color32::WHITE));
    }
}
