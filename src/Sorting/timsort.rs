use super::sort_utils::insertion_sort_range_visual;
use crate::models::SortBar;
use crate::sorting::Operation;
use eframe::egui::Color32;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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
            let extend_to = run_start + min_run;
            insertion_sort_range_visual(bars, run_start, extend_to, tx);
            runs.push((run_start, extend_to));
            i = extend_to;
        } else if run_len < min_run {
            // Already >= min_run, or this is the final tail (< min_run)
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
    let left: Vec<_> = bars[start..mid].to_vec();
    let right: Vec<_> = bars[mid..end].to_vec();
    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    // highlight
    for idx in start..end {
        let _ = tx.send(Operation::SetColor(idx, Color32::YELLOW));
    }
    thread::sleep(Duration::from_millis(30));

    // merge loop
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

    // drain leftovers
    while i < left.len() {
        bars[k] = left[i].clone();
        let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
        thread::sleep(Duration::from_millis(5));
        i += 1;
        k += 1;
    }
    while j < right.len() {
        bars[k] = right[j].clone();
        let _ = tx.send(Operation::SetColor(k, Color32::GREEN));
        thread::sleep(Duration::from_millis(5));
        j += 1;
        k += 1;
    }

    // reset
    for idx in start..end {
        let _ = tx.send(Operation::SetColor(idx, Color32::WHITE));
    }
}
