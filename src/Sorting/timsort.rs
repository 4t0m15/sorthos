use super::sort_utils::insertion_sort_range_visual;
use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::cmp;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Minimal but correct TimSort for SortBar with visual feedback.
/// - Detects natural runs (ascending/descending)
/// - Extends short runs to MIN_MERGE with insertion sort
/// - Maintains a stack of runs and merges according to TimSort invariants
/// - Uses stable merge
pub fn tim_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    if n <= 1 {
        return;
    }
    const MIN_MERGE: usize = 32;

    // Step 1: Detect runs and extend to MIN_MERGE
    let mut runs = Vec::new();
    let mut i = 0;
    while i < n {
        let run_start = i;
        // Detect ascending run
        if i + 1 < n && bars[i].value <= bars[i + 1].value {
            i += 1;
            while i + 1 < n && bars[i].value <= bars[i + 1].value {
                i += 1;
            }
        } else if i + 1 < n {
            // Detect descending run and reverse it
            i += 1;
            while i + 1 < n && bars[i].value > bars[i + 1].value {
                i += 1;
            }
            bars[run_start..=i].reverse();
        }
        let run_end = i + 1;
        let run_len = run_end - run_start;
        // Extend short run to MIN_MERGE with insertion sort
        let min_run = cmp::min(MIN_MERGE, n - run_start);
        if run_len < min_run {
            let right = cmp::min(run_start + min_run, n);
            insertion_sort_range_visual(bars, run_start, right, tx);
            runs.push((run_start, right));
            i = right;
        } else {
            runs.push((run_start, run_end));
            i = run_end;
        }
    }

    // Step 2: Stack-based merging according to TimSort invariants
    // Fix: check all triples from bottom up, only merge last two if no invariant merge
    while runs.len() > 1 {
        let mut merged = false;
        // Check all triples from bottom up
        let mut idx = if runs.len() >= 3 { runs.len() - 3 } else { 0 };
        while runs.len() >= 3 {
            let mut did_merge = false;
            idx = if runs.len() >= 3 { runs.len() - 3 } else { 0 };
            while idx < runs.len() - 2 {
                let len1 = runs[idx].1 - runs[idx].0;
                let len2 = runs[idx + 1].1 - runs[idx + 1].0;
                let len3 = runs[idx + 2].1 - runs[idx + 2].0;
                if len1 <= len2 + len3 || len2 <= len3 {
                    // Merge the smaller of the last two runs
                    if len1 < len3 {
                        merge_visual(bars, runs[idx].0, runs[idx].1, runs[idx + 1].1, tx);
                        runs[idx].1 = runs[idx + 1].1;
                        runs.remove(idx + 1);
                    } else {
                        merge_visual(bars, runs[idx + 1].0, runs[idx + 1].1, runs[idx + 2].1, tx);
                        runs[idx + 1].1 = runs[idx + 2].1;
                        runs.remove(idx + 2);
                    }
                    merged = true;
                    did_merge = true;
                    break; // restart from bottom after merge
                }
                idx += 1;
            }
            if !did_merge {
                break;
            }
        }
        // Only merge last two runs if no invariant merge occurred
        if !merged && runs.len() > 1 {
            let last = runs.len() - 1;
            merge_visual(bars, runs[last - 1].0, runs[last - 1].1, runs[last].1, tx);
            runs[last - 1].1 = runs[last].1;
            runs.pop();
        }
    }

    // Final visual reset
    for idx in 0..n {
        let _ = tx.send(Operation::SetColor(idx, Color32::WHITE));
    }
}

/// Stable merge with visual feedback: merges bars[start..mid] and bars[mid..end]
fn merge_visual(
    bars: &mut Vec<SortBar>,
    start: usize,
    mid: usize,
    end: usize,
    tx: &mpsc::Sender<Operation>,
) {
    let left: Vec<_> = bars[start..mid].iter().cloned().collect();
    let right: Vec<_> = bars[mid..end].iter().cloned().collect();
    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    // Highlight the ranges being merged
    for idx in start..end {
        let _ = tx.send(Operation::SetColor(idx, Color32::YELLOW));
    }
    thread::sleep(Duration::from_millis(30));

    while i < left.len() && j < right.len() {
        let left_idx = start + i;
        let right_idx = mid + j;
        let _ = tx.send(Operation::Compare(left_idx, right_idx));
        thread::sleep(Duration::from_millis(10));
        if left[i].value <= right[j].value {
            bars[k] = left[i].clone();
            let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
            i += 1;
        } else {
            bars[k] = right[j].clone();
            let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
            j += 1;
        }
        thread::sleep(Duration::from_millis(10));
        k += 1;
    }
    while i < left.len() {
        bars[k] = left[i].clone();
        let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
        i += 1;
        k += 1;
        thread::sleep(Duration::from_millis(5));
    }
    while j < right.len() {
        bars[k] = right[j].clone();
        let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
        j += 1;
        k += 1;
        thread::sleep(Duration::from_millis(5));
    }
    // Reset colors after merge
    for idx in start..end {
        let _ = tx.send(Operation::SetColor(idx, Color32::WHITE));
    }
}
