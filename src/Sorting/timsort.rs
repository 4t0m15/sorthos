use super::sort_utils::insertion_sort_range_visual;
use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;

/// Compute the “min run” for TimSort (between 32 and 64)
fn compute_min_run(mut n: usize) -> usize {
    let mut r = 0;
    while n >= 64 {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

/// Minimal but correct TimSort for SortBar with visual feedback.
pub fn tim_sort(bars: &mut Vec<SortBar>, tx: &mpsc::Sender<Operation>) {
    let n = bars.len();
    if n <= 1 {
        return;
    }

    // STEP 0: compute min_run exactly once
    let min_run = compute_min_run(n);

    // STEP 1: detect natural runs & extend short ones with insertion sort
    let mut runs = Vec::new();
    let mut i = 0;
    while i < n {
        let run_start = i;

        // ascending run
        if i + 1 < n && bars[i].value <= bars[i + 1].value {
            i += 1;
            while i + 1 < n && bars[i].value <= bars[i + 1].value {
                i += 1;
            }
        }
        // descending run
        else if i + 1 < n {
            i += 1;
            while i + 1 < n && bars[i].value > bars[i + 1].value {
                i += 1;
            }
            bars[run_start..=i].reverse();
        }

        let run_end = i + 1;
        let run_len = run_end - run_start;
        let remaining = n - run_start;

        // Only force-extend if there are at least min_run elements left after this run
        if run_len < min_run && remaining >= min_run {
            // Not enough elements for a natural run – extend using insertion sort
            let extend_to = run_start + min_run;
            insertion_sort_range_visual(bars, run_start, extend_to, tx);
            runs.push((run_start, extend_to));
            i = extend_to;
        } else if run_len < min_run {
            // Final tail smaller than MIN_RUN
            runs.push((run_start, run_end));
            i = run_end;
        } else {
            // Natural run already at least MIN_RUN in length
            runs.push((run_start, run_end));
            i = run_end;
        }
    }

    // STEP 2: collapse runs according to TimSort invariants
    while runs.len() > 1 {
        let mut merged = false;

        // collapse any invariant-violating triples
        loop {
            let mut did_merge = false;
            let mut idx = runs.len().saturating_sub(3);

            while idx < runs.len().saturating_sub(2) {
                let len1 = runs[idx].1 - runs[idx].0;
                let len2 = runs[idx + 1].1 - runs[idx + 1].0;
                let len3 = runs[idx + 2].1 - runs[idx + 2].0;

                if len1 <= len2 + len3 || len2 <= len3 {
                    // merge the smaller of the two adjacent runs
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
                    break;
                }
                idx += 1;
            }

            if !did_merge {
                break;
            }
        }

        // if we never did an invariant merge, merge the last two runs
        if !merged && runs.len() > 1 {
            let last = runs.len() - 1;
            merge_visual(bars, runs[last - 1].0, runs[last - 1].1, runs[last].1, tx);
            runs[last - 1].1 = runs[last].1;
            runs.pop();
        }
    }

    // STEP 3: final color reset
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
    let left_len = mid - start;
    let right_len = end - mid;

    // Highlight the merging range
    for x in start..end {
        let _ = tx.send(Operation::SetColor(x, Color32::YELLOW));
    }
    std::thread::sleep(std::time::Duration::from_millis(20));

    if left_len <= right_len {
        // Merge left (smaller side) using temp buffer
        let left = bars[start..mid].to_vec();
        let mut i = 0;
        let mut j = mid;
        let mut k = start;

        while i < left_len && j < end {
            let _ = tx.send(Operation::Compare(start + i, j));
            std::thread::sleep(std::time::Duration::from_millis(10));
            if left[i].value <= bars[j].value {
                bars[k] = left[i].clone();
                i += 1;
            } else {
                bars[k] = bars[j].clone();
                j += 1;
            }
            let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
            std::thread::sleep(std::time::Duration::from_millis(10));
            k += 1;
        }
        // Copy remaining left elements
        while i < left_len {
            bars[k] = left[i].clone();
            let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
            std::thread::sleep(std::time::Duration::from_millis(5));
            i += 1;
            k += 1;
        }
    } else {
        // Always copy the right run to a temporary buffer
        let right = bars[mid..end].to_vec();
        let mut i = mid.wrapping_sub(1); // Start from last element of left run
        let mut j = right_len.wrapping_sub(1); // Last element of right buffer
        let mut k = end.wrapping_sub(1); // Start from end of merged array

        // Process while both runs have elements
        while i >= start && j < right.len() {
            let _ = tx.send(Operation::Compare(i, mid + j));
            std::thread::sleep(std::time::Duration::from_millis(10));
            if bars[i].value > right[j].value {
                bars[k] = bars[i].clone();
                i = i.wrapping_sub(1); // Move left in original array
            } else {
                bars[k] = right[j].clone();
                j = j.wrapping_sub(1); // Move left in temp buffer
            }
            let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
            std::thread::sleep(std::time::Duration::from_millis(10));
            k = k.wrapping_sub(1);
        }

        // Copy any remaining right elements
        while j < right.len() {
            bars[k] = right[j].clone();
            let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
            std::thread::sleep(std::time::Duration::from_millis(5));
            j = j.wrapping_sub(1);
            k = k.wrapping_sub(1);
        }
    }

    // Reset all bars to white
    for x in start..end {
        let _ = tx.send(Operation::SetColor(x, Color32::WHITE));
    }
}
